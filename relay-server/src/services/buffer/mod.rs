//! Types for buffering envelopes.

use std::error::Error;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::num::NonZeroU8;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;

use chrono::DateTime;
use chrono::Utc;
use fnv::FnvHasher;
use relay_base_schema::project::ProjectKey;
use relay_config::Config;
use relay_system::ServiceRunner;
use relay_system::{Addr, FromMessage, Interface, NoResponse, Receiver, Service};
use relay_system::{Controller, Shutdown};
use tokio::sync::mpsc::Permit;
use tokio::sync::{mpsc, watch};
use tokio::time::{timeout, Instant};

use crate::envelope::Envelope;
use crate::services::buffer::envelope_buffer::Peek;
use crate::services::global_config;
use crate::services::outcome::DiscardReason;
use crate::services::outcome::Outcome;
use crate::services::outcome::TrackOutcome;
use crate::services::processor::ProcessingGroup;
use crate::services::projects::cache::{legacy, ProjectCacheHandle, ProjectChange};
use crate::services::test_store::TestStore;
use crate::statsd::RelayTimers;
use crate::statsd::{RelayCounters, RelayGauges, RelayHistograms};
use crate::utils::ManagedEnvelope;
use crate::MemoryChecker;
use crate::MemoryStat;

// pub for benchmarks
pub use envelope_buffer::EnvelopeBufferError;
// pub for benchmarks
pub use envelope_buffer::PolymorphicEnvelopeBuffer;
// pub for benchmarks
pub use envelope_stack::sqlite::SqliteEnvelopeStack;
// pub for benchmarks
pub use envelope_stack::EnvelopeStack;
// pub for benchmarks
pub use envelope_store::sqlite::SqliteEnvelopeStore;

pub use common::ProjectKeyPair;

mod common;
mod envelope_buffer;
mod envelope_stack;
mod envelope_store;
mod stack_provider;
mod testutils;

/// Message interface for [`EnvelopeBufferService`].
#[derive(Debug)]
pub enum EnvelopeBuffer {
    /// A fresh envelope that gets pushed into the buffer by the request handler.
    Push(Box<Envelope>),
    /// Informs the service that a project has no valid project state and must be marked as not ready.
    ///
    /// This happens when an envelope was sent to the project cache, but one of the necessary project
    /// state has expired. The envelope is pushed back into the envelope buffer.
    NotReady(ProjectKey, Box<Envelope>),
}

impl EnvelopeBuffer {
    fn name(&self) -> &'static str {
        match &self {
            EnvelopeBuffer::Push(_) => "push",
            EnvelopeBuffer::NotReady(..) => "project_not_ready",
        }
    }
}

impl Interface for EnvelopeBuffer {}

impl FromMessage<Self> for EnvelopeBuffer {
    type Response = NoResponse;

    fn from_message(message: Self, _: ()) -> Self {
        message
    }
}

/// Abstraction that wraps a list of [`ObservableEnvelopeBuffer`]s to which [`Envelope`] are routed
/// based on their [`ProjectKeyPair`].
#[derive(Debug, Clone)]
pub struct PartitionedEnvelopeBuffer {
    buffers: Arc<Vec<ObservableEnvelopeBuffer>>,
}

impl PartitionedEnvelopeBuffer {
    /// Creates a [`PartitionedEnvelopeBuffer`] with no partitions.
    #[cfg(test)]
    pub fn empty() -> Self {
        Self {
            buffers: Arc::new(Vec::new()),
        }
    }

    /// Creates a new [`PartitionedEnvelopeBuffer`] by instantiating inside all the necessary
    /// [`ObservableEnvelopeBuffer`]s.
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        partitions: NonZeroU8,
        config: Arc<Config>,
        memory_stat: MemoryStat,
        global_config_rx: watch::Receiver<global_config::Status>,
        envelopes_tx: mpsc::Sender<legacy::DequeuedEnvelope>,
        project_cache_handle: ProjectCacheHandle,
        outcome_aggregator: Addr<TrackOutcome>,
        test_store: Addr<TestStore>,
        runner: &mut ServiceRunner,
    ) -> Self {
        let mut envelope_buffers = Vec::with_capacity(partitions.get() as usize);
        for partition_id in 0..partitions.get() {
            let envelope_buffer = EnvelopeBufferService::new(
                partition_id,
                config.clone(),
                memory_stat.clone(),
                global_config_rx.clone(),
                Services {
                    envelopes_tx: envelopes_tx.clone(),
                    project_cache_handle: project_cache_handle.clone(),
                    outcome_aggregator: outcome_aggregator.clone(),
                    test_store: test_store.clone(),
                },
            )
            .map(|b| b.start_in(runner));

            if let Some(envelope_buffer) = envelope_buffer {
                envelope_buffers.push(envelope_buffer);
            }
        }

        Self {
            buffers: Arc::new(envelope_buffers),
        }
    }

    /// Returns the [`ObservableEnvelopeBuffer`] to which [`Envelope`]s having the supplied
    /// [`ProjectKeyPair`] will be sent.
    ///
    /// The rationale of using this partitioning strategy is to reduce memory usage across buffers
    /// since each individual buffer will only take care of a subset of projects.
    pub fn buffer(&self, project_key_pair: ProjectKeyPair) -> Option<&ObservableEnvelopeBuffer> {
        let mut hasher = FnvHasher::default();
        project_key_pair.hash(&mut hasher);
        let buffer_index = (hasher.finish() % self.buffers.len() as u64) as usize;
        let buffer = self.buffers.get(buffer_index);
        buffer
    }

    /// Returns `true` if all [`ObservableEnvelopeBuffer`]s have capacity to get new [`Envelope`]s.
    ///
    /// If no buffers are specified, the function returns `true`, assuming that there is capacity
    /// if the buffer is not setup.
    pub fn has_capacity(&self) -> bool {
        if self.buffers.is_empty() {
            return true;
        }

        self.buffers.iter().all(|buffer| buffer.has_capacity())
    }
}

/// Contains the services [`Addr`] and a watch channel to observe its state.
///
/// This allows outside observers to check the capacity without having to send a message.
///
// NOTE: This pattern of combining an Addr with some observable state could be generalized into
// `Service` itself.
#[derive(Debug, Clone)]
pub struct ObservableEnvelopeBuffer {
    addr: Addr<EnvelopeBuffer>,
    has_capacity: Arc<AtomicBool>,
}

impl ObservableEnvelopeBuffer {
    /// Returns the address of the buffer service.
    pub fn addr(&self) -> Addr<EnvelopeBuffer> {
        self.addr.clone()
    }

    /// Returns `true` if the buffer has the capacity to accept more elements.
    pub fn has_capacity(&self) -> bool {
        self.has_capacity.load(Ordering::Relaxed)
    }
}

/// Services that the buffer service communicates with.
#[derive(Clone)]
pub struct Services {
    /// Bounded channel used exclusively to handle backpressure when sending envelopes to the
    /// project cache.
    pub envelopes_tx: mpsc::Sender<legacy::DequeuedEnvelope>,
    pub project_cache_handle: ProjectCacheHandle,
    pub outcome_aggregator: Addr<TrackOutcome>,
    pub test_store: Addr<TestStore>,
}

/// Spool V2 service which buffers envelopes and forwards them to the project cache when a project
/// becomes ready.
pub struct EnvelopeBufferService {
    partition_id: u8,
    config: Arc<Config>,
    memory_stat: MemoryStat,
    global_config_rx: watch::Receiver<global_config::Status>,
    services: Services,
    has_capacity: Arc<AtomicBool>,
    sleep: Duration,
}

/// The maximum amount of time between evaluations of dequeue conditions.
///
/// Some condition checks are sync (`has_capacity`), so cannot be awaited. The sleep in cancelled
/// whenever a new message or a global config update comes in.
const DEFAULT_SLEEP: Duration = Duration::from_secs(1);

impl EnvelopeBufferService {
    /// Creates a memory or disk based [`EnvelopeBufferService`], depending on the given config.
    ///
    /// NOTE: until the V1 spooler implementation is removed, this function returns `None`
    /// if V2 spooling is not configured.
    pub fn new(
        partition_id: u8,
        config: Arc<Config>,
        memory_stat: MemoryStat,
        global_config_rx: watch::Receiver<global_config::Status>,
        services: Services,
    ) -> Option<Self> {
        config.spool_v2().then(|| Self {
            partition_id,
            config,
            memory_stat,
            global_config_rx,
            services,
            has_capacity: Arc::new(AtomicBool::new(true)),
            sleep: Duration::ZERO,
        })
    }

    /// Returns both the [`Addr`] to this service, and a reference to the capacity flag.
    pub fn start_in(self, runner: &mut ServiceRunner) -> ObservableEnvelopeBuffer {
        let has_capacity = self.has_capacity.clone();

        let addr = runner.start(self);
        ObservableEnvelopeBuffer { addr, has_capacity }
    }

    /// Wait for the configured amount of time and make sure the project cache is ready to receive.
    async fn ready_to_pop(
        &mut self,
        buffer: &PolymorphicEnvelopeBuffer,
        dequeue: bool,
    ) -> Option<Permit<legacy::DequeuedEnvelope>> {
        relay_statsd::metric!(
            counter(RelayCounters::BufferReadyToPop) += 1,
            status = "checking"
        );

        self.system_ready(buffer, dequeue).await;

        relay_statsd::metric!(
            counter(RelayCounters::BufferReadyToPop) += 1,
            status = "system_ready"
        );

        if self.sleep > Duration::ZERO {
            tokio::time::sleep(self.sleep).await;
        }

        relay_statsd::metric!(
            counter(RelayCounters::BufferReadyToPop) += 1,
            status = "slept"
        );

        let permit = self.services.envelopes_tx.reserve().await.ok();

        relay_statsd::metric!(
            counter(RelayCounters::BufferReadyToPop) += 1,
            status = "checked"
        );

        permit
    }

    /// Waits until preconditions for unspooling are met.
    ///
    /// - We should not pop from disk into memory when relay's overall memory capacity
    ///   has been reached.
    /// - We need a valid global config to unspool.
    async fn system_ready(&self, buffer: &PolymorphicEnvelopeBuffer, dequeue: bool) {
        loop {
            // We should not unspool from external storage if memory capacity has been reached.
            // But if buffer storage is in memory, unspooling can reduce memory usage.
            let memory_ready = buffer.is_memory() || self.memory_ready();
            let global_config_ready = self.global_config_rx.borrow().is_ready();

            if memory_ready && global_config_ready && dequeue {
                return;
            }
            tokio::time::sleep(DEFAULT_SLEEP).await;
        }
    }

    fn memory_ready(&self) -> bool {
        self.memory_stat.memory().used_percent()
            <= self.config.spool_max_backpressure_memory_percent()
    }

    /// Tries to pop an envelope for a ready project.
    async fn try_pop<'a>(
        config: &Config,
        buffer: &mut PolymorphicEnvelopeBuffer,
        services: &Services,
        envelopes_tx_permit: Permit<'a, legacy::DequeuedEnvelope>,
    ) -> Result<Duration, EnvelopeBufferError> {
        let sleep = match buffer.peek().await? {
            Peek::Empty => {
                relay_statsd::metric!(
                    counter(RelayCounters::BufferTryPop) += 1,
                    peek_result = "empty"
                );

                Duration::MAX // wait for reset by `handle_message`.
            }
            Peek::Ready { last_received_at }
            | Peek::NotReady {
                last_received_at, ..
            } if is_expired(last_received_at, config) => {
                let envelope = buffer
                    .pop()
                    .await?
                    .expect("Element disappeared despite exclusive excess");

                Self::drop_expired(envelope, services);

                Duration::ZERO // try next pop immediately
            }
            Peek::Ready { .. } => {
                relay_log::trace!("EnvelopeBufferService: popping envelope");
                relay_statsd::metric!(
                    counter(RelayCounters::BufferTryPop) += 1,
                    peek_result = "ready"
                );
                let envelope = buffer
                    .pop()
                    .await?
                    .expect("Element disappeared despite exclusive excess");
                envelopes_tx_permit.send(legacy::DequeuedEnvelope(envelope));

                println!("READY");

                Duration::ZERO // try next pop immediately
            }
            Peek::NotReady {
                project_key_pair,
                next_project_fetch,
                last_received_at: _,
            } => {
                relay_log::trace!("EnvelopeBufferService: project(s) of envelope not ready");
                relay_statsd::metric!(
                    counter(RelayCounters::BufferTryPop) += 1,
                    peek_result = "not_ready"
                );

                println!("NOT READY");

                // We want to fetch the configs again, only if some time passed between the last
                // peek of this not ready project key pair and the current peek. This is done to
                // avoid flooding the project cache with `UpdateProject` messages.
                if Instant::now() >= next_project_fetch {
                    relay_log::trace!("EnvelopeBufferService: requesting project(s) update");

                    let ProjectKeyPair {
                        own_key,
                        sampling_key,
                    } = project_key_pair;

                    // services.project_cache_handle.fetch(own_key);
                    // if sampling_key != own_key {
                    //     services.project_cache_handle.fetch(sampling_key);
                    // }

                    // Deprioritize the stack to prevent head-of-line blocking and update the next fetch
                    // time.
                    buffer.mark_seen(&project_key_pair, DEFAULT_SLEEP);
                }

                DEFAULT_SLEEP // wait and prioritize handling new messages.
            }
        };

        Ok(sleep)
    }

    fn drop_expired(envelope: Box<Envelope>, services: &Services) {
        let mut managed_envelope = ManagedEnvelope::new(
            envelope,
            services.outcome_aggregator.clone(),
            services.test_store.clone(),
            ProcessingGroup::Ungrouped,
        );
        managed_envelope.reject(Outcome::Invalid(DiscardReason::Timestamp));
    }

    async fn handle_message(
        buffer: &mut PolymorphicEnvelopeBuffer,
        services: &Services,
        message: EnvelopeBuffer,
    ) {
        match message {
            EnvelopeBuffer::Push(envelope) => {
                // NOTE: This function assumes that a project state update for the relevant
                // projects was already triggered (see XXX).
                // For better separation of concerns, this prefetch should be triggered from here
                // once buffer V1 has been removed.
                relay_log::trace!("EnvelopeBufferService: received push message");
                Self::push(buffer, envelope).await;
            }
            EnvelopeBuffer::NotReady(project_key, envelope) => {
                relay_log::trace!(
                    "EnvelopeBufferService: received project not ready message for project key {}",
                    &project_key
                );
                relay_statsd::metric!(counter(RelayCounters::BufferEnvelopesReturned) += 1);
                Self::push(buffer, envelope).await;
                let project = services.project_cache_handle.get(project_key);
                buffer.mark_ready(&project_key, !project.state().is_pending());
            }
        };
    }

    async fn handle_shutdown(buffer: &mut PolymorphicEnvelopeBuffer, message: Shutdown) -> bool {
        // We gracefully shut down only if the shutdown has a timeout.
        if let Some(shutdown_timeout) = message.timeout {
            relay_log::trace!("EnvelopeBufferService: shutting down gracefully");

            let shutdown_result = timeout(shutdown_timeout, buffer.shutdown()).await;
            match shutdown_result {
                Ok(shutdown_result) => {
                    return shutdown_result;
                }
                Err(error) => {
                    relay_log::error!(
                    error = &error as &dyn Error,
                    "the envelope buffer didn't shut down in time, some envelopes might be lost",
                );
                }
            }
        }

        false
    }

    async fn push(buffer: &mut PolymorphicEnvelopeBuffer, envelope: Box<Envelope>) {
        if let Err(e) = buffer.push(envelope).await {
            relay_log::error!(
                error = &e as &dyn std::error::Error,
                "failed to push envelope"
            );
        }
    }

    fn update_observable_state(&self, buffer: &mut PolymorphicEnvelopeBuffer) {
        self.has_capacity
            .store(buffer.has_capacity(), Ordering::Relaxed);
    }
}

fn is_expired(last_received_at: DateTime<Utc>, config: &Config) -> bool {
    (Utc::now() - last_received_at)
        .to_std()
        .is_ok_and(|age| age > config.spool_envelopes_max_age())
}

impl Service for EnvelopeBufferService {
    type Interface = EnvelopeBuffer;

    async fn run(mut self, mut rx: Receiver<Self::Interface>) {
        let config = self.config.clone();
        let memory_checker = MemoryChecker::new(self.memory_stat.clone(), config.clone());
        let mut global_config_rx = self.global_config_rx.clone();
        let services = self.services.clone();

        let dequeue = Arc::<AtomicBool>::new(true.into());
        let dequeue1 = dequeue.clone();

        let mut buffer =
            PolymorphicEnvelopeBuffer::from_config(self.partition_id, &config, memory_checker)
                .await
                .expect("failed to start the envelope buffer service");

        buffer.initialize().await;

        let mut shutdown = Controller::shutdown_handle();
        let mut project_changes = self.services.project_cache_handle.changes();

        #[cfg(unix)]
        relay_system::spawn!(async move {
            use tokio::signal::unix::{signal, SignalKind};
            let Ok(mut signal) = signal(SignalKind::user_defined1()) else {
                return;
            };
            while let Some(()) = signal.recv().await {
                let deq = !dequeue1.load(Ordering::Relaxed);
                dequeue1.store(deq, Ordering::Relaxed);
                relay_log::info!("SIGUSR1 receive, dequeue={}", deq);
            }
        });

        let mut last_metrics_time = SystemTime::now();
        let mut total_idle_time = Duration::ZERO;
        let mut total_busy_time = Duration::ZERO;
        let mut total_push_time = Duration::ZERO;
        let mut total_pop_time = Duration::ZERO;
        let mut total_project_cache_time = Duration::ZERO;
        let mut total_push_bytes = 0;
        let mut push_count = 0u64;
        let mut last_loop_time = Instant::now();

        relay_log::info!("EnvelopeBufferService: starting");
        loop {
            let used_capacity =
                self.services.envelopes_tx.max_capacity() - self.services.envelopes_tx.capacity();
            relay_statsd::metric!(
                histogram(RelayHistograms::BufferBackpressureEnvelopesCount) = used_capacity as u64
            );

            // Track idle time since last iteration
            let idle_duration = last_loop_time.elapsed();
            total_idle_time += idle_duration;

            // Print metrics every second
            if let Ok(elapsed) = last_metrics_time.elapsed() {
                if elapsed.as_secs() >= 1 {
                    let avg_push_duration = if push_count > 0 {
                        total_push_time.div_f64(push_count as f64)
                    } else {
                        Duration::ZERO
                    };

                    let envelope_stacks_count = match &buffer {
                        PolymorphicEnvelopeBuffer::InMemory(buffer) => buffer.priority_queue.len(),
                        PolymorphicEnvelopeBuffer::Sqlite(buffer) => buffer.priority_queue.len(),
                    };

                    println!(
                        "Buffer {} metrics:\n  Queue size: {}\n  Idle time: {:.2?}\n  Busy time: {:.2?}\n  Push time: {:.2?}\n  Pop time: {:.2?}\n  Cache time: {:.2?}\n  Avg push duration: {:.2?}\n  Pushed kib/s: {}\n  Push count: {}\n  Envelope stacks count: {}\n",
                        self.partition_id,
                        rx.queue_size.load(Ordering::Relaxed),
                        total_idle_time,
                        total_busy_time,
                        total_push_time,
                        total_pop_time,
                        total_project_cache_time,
                        avg_push_duration,
                        total_push_bytes as f64 / 1024f64,
                        push_count,
                        envelope_stacks_count
                    );

                    // Reset counters after logging
                    total_idle_time = Duration::ZERO;
                    total_busy_time = Duration::ZERO;
                    total_push_time = Duration::ZERO;
                    total_pop_time = Duration::ZERO;
                    total_project_cache_time = Duration::ZERO;
                    total_push_bytes = 0;
                    push_count = 0;
                    last_metrics_time = SystemTime::now();

                    // Track metrics
                    buffer.track().await;
                }
            }

            let mut sleep = Duration::MAX;
            let start = Instant::now();
            tokio::select! {
                // NOTE: we do not select a bias here.
                // On the one hand, we might want to prioritize dequeuing over enqueuing
                // so we do not exceed the buffer capacity by starving the dequeue.
                // on the other hand, prioritizing old messages violates the LIFO design.
                Some(permit) = self.ready_to_pop(&buffer, false) => {
                    total_idle_time += start.elapsed();
                    let busy_start = Instant::now();
                    match Self::try_pop(&config, &mut buffer, &services, permit).await {
                        Ok(new_sleep) => {
                            sleep = new_sleep;
                        }
                        Err(error) => {
                            relay_log::error!(
                                error = &error as &dyn std::error::Error,
                                "failed to pop envelope"
                            );
                        }
                    }
                   total_busy_time += busy_start.elapsed();
                   total_pop_time += start.elapsed();
                }
                change = project_changes.recv() => {
                    total_idle_time += start.elapsed();
                    let busy_start = Instant::now();
                    if let Ok(ProjectChange::Ready(project_key)) = change {
                        buffer.mark_ready(&project_key, true);
                    }
                    sleep = Duration::ZERO;
                    total_busy_time += busy_start.elapsed();
                }
                Some(message) = rx.recv() => {
                    total_idle_time += start.elapsed();

                    let envelope_size = match &message {
                        EnvelopeBuffer::Push(envelope) => envelope.items().map(|i| i.len()).sum::<usize>(),
                        EnvelopeBuffer::NotReady(_,_) => 0
                    };

                    let busy_start = Instant::now();
                    let message_name = message.name();
                    Self::handle_message(&mut buffer, &services, message).await;
                    sleep = Duration::ZERO;
                    let elapsed = busy_start.elapsed();
                    // Track push time and count separately
                    if message_name == "push" {
                        total_push_time += elapsed;
                        total_push_bytes += envelope_size;
                        push_count += 1;
                    }
                    total_busy_time += elapsed;
                }
                shutdown = shutdown.notified() => {
                    total_idle_time += start.elapsed();
                    let busy_start = Instant::now();
                    if Self::handle_shutdown(&mut buffer, shutdown).await {
                        break;
                    }
                    total_busy_time += busy_start.elapsed();
                }
                Ok(()) = global_config_rx.changed() => {
                    total_idle_time += start.elapsed();
                    let busy_start = Instant::now();
                    sleep = Duration::ZERO;
                    total_busy_time += busy_start.elapsed();
                    total_project_cache_time += start.elapsed();
                }
                else => break,
            }

            self.sleep = sleep;
            self.update_observable_state(&mut buffer);
            last_loop_time = Instant::now();
        }

        relay_log::info!("EnvelopeBufferService: stopping");
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use relay_dynamic_config::GlobalConfig;
    use relay_quotas::DataCategory;
    use std::time::Duration;
    use tokio::sync::mpsc;
    use uuid::Uuid;

    use crate::services::projects::project::ProjectState;
    use crate::testutils::new_envelope;
    use crate::MemoryStat;

    use super::*;

    struct EnvelopeBufferServiceResult {
        service: EnvelopeBufferService,
        global_tx: watch::Sender<global_config::Status>,
        envelopes_rx: mpsc::Receiver<legacy::DequeuedEnvelope>,
        project_cache_handle: ProjectCacheHandle,
        outcome_aggregator_rx: mpsc::UnboundedReceiver<TrackOutcome>,
    }

    fn envelope_buffer_service(
        config_json: Option<serde_json::Value>,
        global_config_status: global_config::Status,
    ) -> EnvelopeBufferServiceResult {
        relay_log::init_test!();

        let config_json = config_json.unwrap_or(serde_json::json!({
            "spool": {
                "envelopes": {
                    "version": "experimental"
                }
            }
        }));
        let config = Arc::new(Config::from_json_value(config_json).unwrap());

        let memory_stat = MemoryStat::default();
        let (global_tx, global_rx) = watch::channel(global_config_status);
        let (envelopes_tx, envelopes_rx) = mpsc::channel(5);
        let (outcome_aggregator, outcome_aggregator_rx) = Addr::custom();
        let project_cache_handle = ProjectCacheHandle::for_test();

        let envelope_buffer_service = EnvelopeBufferService::new(
            0,
            config,
            memory_stat,
            global_rx,
            Services {
                envelopes_tx,
                project_cache_handle: project_cache_handle.clone(),
                outcome_aggregator,
                test_store: Addr::dummy(),
            },
        )
        .unwrap();

        EnvelopeBufferServiceResult {
            service: envelope_buffer_service,
            global_tx,
            envelopes_rx,
            project_cache_handle,
            outcome_aggregator_rx,
        }
    }

    #[tokio::test(start_paused = true)]
    async fn capacity_is_updated() {
        let EnvelopeBufferServiceResult {
            service,
            global_tx: _global_tx,
            envelopes_rx: _envelopes_rx,
            project_cache_handle,
            outcome_aggregator_rx: _outcome_aggregator_rx,
        } = envelope_buffer_service(None, global_config::Status::Pending);

        service.has_capacity.store(false, Ordering::Relaxed);

        let ObservableEnvelopeBuffer { has_capacity, .. } =
            service.start_in(&mut ServiceRunner::new());
        assert!(!has_capacity.load(Ordering::Relaxed));

        tokio::time::advance(Duration::from_millis(100)).await;

        let some_project_key = ProjectKey::parse("a94ae32be2584e0bbd7a4cbb95971fee").unwrap();
        project_cache_handle.test_set_project_state(some_project_key, ProjectState::Disabled);

        tokio::time::advance(Duration::from_millis(100)).await;

        assert!(has_capacity.load(Ordering::Relaxed));
    }

    #[tokio::test(start_paused = true)]
    async fn pop_requires_global_config() {
        let EnvelopeBufferServiceResult {
            service,
            global_tx,
            envelopes_rx,
            project_cache_handle,
            outcome_aggregator_rx: _outcome_aggregator_rx,
            ..
        } = envelope_buffer_service(None, global_config::Status::Pending);

        let addr = service.start_detached();

        let envelope = new_envelope(false, "foo");
        let project_key = envelope.meta().public_key();
        addr.send(EnvelopeBuffer::Push(envelope.clone()));
        project_cache_handle.test_set_project_state(project_key, ProjectState::Disabled);

        tokio::time::sleep(Duration::from_millis(1000)).await;

        assert_eq!(envelopes_rx.len(), 0);

        global_tx.send_replace(global_config::Status::Ready(Arc::new(
            GlobalConfig::default(),
        )));

        tokio::time::sleep(Duration::from_millis(1000)).await;

        assert_eq!(envelopes_rx.len(), 1);
    }

    #[tokio::test(start_paused = true)]
    async fn pop_requires_memory_capacity() {
        let EnvelopeBufferServiceResult {
            service,
            envelopes_rx,
            project_cache_handle,
            outcome_aggregator_rx: _outcome_aggregator_rx,
            global_tx: _global_tx,
            ..
        } = envelope_buffer_service(
            Some(serde_json::json!({
                "spool": {
                    "envelopes": {
                        "version": "experimental",
                        "path": std::env::temp_dir().join(Uuid::new_v4().to_string()),
                    }
                },
                "health": {
                    "max_memory_bytes": 0,
                }
            })),
            global_config::Status::Ready(Arc::new(GlobalConfig::default())),
        );

        let addr = service.start_detached();

        let envelope = new_envelope(false, "foo");
        let project_key = envelope.meta().public_key();
        addr.send(EnvelopeBuffer::Push(envelope.clone()));
        project_cache_handle.test_set_project_state(project_key, ProjectState::Disabled);

        tokio::time::sleep(Duration::from_millis(1000)).await;

        assert_eq!(envelopes_rx.len(), 0);
    }

    #[tokio::test(start_paused = true)]
    async fn old_envelope_is_dropped() {
        let EnvelopeBufferServiceResult {
            service,
            envelopes_rx,
            project_cache_handle: _project_cache_handle,
            mut outcome_aggregator_rx,
            global_tx: _global_tx,
        } = envelope_buffer_service(
            Some(serde_json::json!({
                "spool": {
                    "envelopes": {
                        "version": "experimental",
                        "max_envelope_delay_secs": 1,
                    }
                }
            })),
            global_config::Status::Ready(Arc::new(GlobalConfig::default())),
        );

        let config = service.config.clone();
        let addr = service.start_detached();

        tokio::time::sleep(Duration::from_millis(100)).await;

        let mut envelope = new_envelope(false, "foo");
        envelope.meta_mut().set_received_at(
            Utc::now()
                - chrono::Duration::seconds(2 * config.spool_envelopes_max_age().as_secs() as i64),
        );
        addr.send(EnvelopeBuffer::Push(envelope));

        tokio::time::sleep(Duration::from_millis(100)).await;

        assert_eq!(envelopes_rx.len(), 0);

        let outcome = outcome_aggregator_rx.try_recv().unwrap();
        assert_eq!(outcome.category, DataCategory::TransactionIndexed);
        assert_eq!(outcome.quantity, 1);
    }

    #[tokio::test(start_paused = true)]
    async fn test_update_project() {
        let EnvelopeBufferServiceResult {
            service,
            mut envelopes_rx,
            project_cache_handle,
            global_tx: _global_tx,
            outcome_aggregator_rx: _outcome_aggregator_rx,
        } = envelope_buffer_service(
            None,
            global_config::Status::Ready(Arc::new(GlobalConfig::default())),
        );

        let addr = service.start_detached();

        let envelope = new_envelope(false, "foo");
        let project_key = envelope.meta().public_key();

        tokio::time::sleep(Duration::from_secs(1)).await;

        addr.send(EnvelopeBuffer::Push(envelope.clone()));
        tokio::time::sleep(Duration::from_secs(3)).await;

        let legacy::DequeuedEnvelope(envelope) = envelopes_rx.recv().await.unwrap();

        addr.send(EnvelopeBuffer::NotReady(project_key, envelope));

        tokio::time::sleep(Duration::from_millis(200)).await;
        assert_eq!(project_cache_handle.test_num_fetches(), 2);

        tokio::time::sleep(Duration::from_millis(1300)).await;
        assert_eq!(project_cache_handle.test_num_fetches(), 3);
    }

    #[tokio::test(start_paused = true)]
    async fn output_is_throttled() {
        let EnvelopeBufferServiceResult {
            service,
            mut envelopes_rx,
            project_cache_handle,
            global_tx: _global_tx,
            outcome_aggregator_rx: _outcome_aggregator_rx,
            ..
        } = envelope_buffer_service(
            None,
            global_config::Status::Ready(Arc::new(GlobalConfig::default())),
        );

        let addr = service.start_detached();

        let envelope = new_envelope(false, "foo");
        let project_key = envelope.meta().public_key();
        for _ in 0..10 {
            addr.send(EnvelopeBuffer::Push(envelope.clone()));
        }
        project_cache_handle.test_set_project_state(project_key, ProjectState::Disabled);

        tokio::time::sleep(Duration::from_millis(100)).await;

        let mut messages = vec![];
        envelopes_rx.recv_many(&mut messages, 100).await;

        assert_eq!(
            messages
                .iter()
                .filter(|message| matches!(message, legacy::DequeuedEnvelope(..)))
                .count(),
            5
        );

        tokio::time::sleep(Duration::from_millis(100)).await;

        let mut messages = vec![];
        envelopes_rx.recv_many(&mut messages, 100).await;

        assert_eq!(
            messages
                .iter()
                .filter(|message| matches!(message, legacy::DequeuedEnvelope(..)))
                .count(),
            5
        );
    }
}
