use crate::services::buffer::common::{EnvelopeBufferError, ProjectKeyPair};
use crate::services::buffer::envelope_provider::memory::MemoryEnvelopeProvider;
use crate::services::buffer::envelope_provider::sqlite::SqliteEnvelopeProvider;
use crate::{Envelope, MemoryChecker};
use hashbrown::HashSet;
use relay_config::Config;
use std::fmt;

pub mod memory;
pub mod sqlite;

/// State of the initialization of the [`StackProvider`].
///
/// This state is necessary for initializing resources whenever a [`StackProvider`] is used.
#[derive(Debug)]
pub struct InitializationState {
    pub project_key_pairs: HashSet<ProjectKeyPair>,
}

impl InitializationState {
    /// Create a new [`InitializationState`].
    pub fn new(project_key_pairs: HashSet<ProjectKeyPair>) -> Self {
        Self { project_key_pairs }
    }

    /// Creates a new empty [`InitializationState`].
    pub fn empty() -> Self {
        Self {
            project_key_pairs: HashSet::new(),
        }
    }
}

#[derive(Debug)]
pub enum EnvelopeProvider {
    Memory(MemoryEnvelopeProvider),
    SQLite(SqliteEnvelopeProvider),
}

impl EnvelopeProvider {
    pub fn memory(memory_checker: MemoryChecker) -> Result<Self, EnvelopeBufferError> {
        Ok(Self::Memory(MemoryEnvelopeProvider::new(memory_checker)))
    }

    pub async fn sqlite(config: &Config) -> Result<Self, EnvelopeBufferError> {
        Ok(Self::SQLite(SqliteEnvelopeProvider::new(config).await?))
    }

    pub async fn push(
        &mut self,
        project_key_pair: ProjectKeyPair,
        envelope: Box<Envelope>,
    ) -> Result<(), EnvelopeBufferError> {
        match self {
            EnvelopeProvider::Memory(provider) => provider.push(project_key_pair, envelope).await?,
            EnvelopeProvider::SQLite(provider) => provider.push(project_key_pair, envelope).await?,
        }

        Ok(())
    }

    pub async fn peek(
        &mut self,
        project_key_pair: ProjectKeyPair,
    ) -> Result<Option<&Envelope>, EnvelopeBufferError> {
        let envelope = match self {
            EnvelopeProvider::Memory(provider) => provider.peek(project_key_pair).await?,
            EnvelopeProvider::SQLite(provider) => provider.peek(project_key_pair).await?,
        };

        Ok(envelope)
    }

    pub async fn pop(
        &mut self,
        project_key_pair: ProjectKeyPair,
    ) -> Result<Option<Box<Envelope>>, EnvelopeBufferError> {
        let envelope = match self {
            EnvelopeProvider::Memory(provider) => provider.pop(project_key_pair).await?,
            EnvelopeProvider::SQLite(provider) => provider.pop(project_key_pair).await?,
        };

        Ok(envelope)
    }
}

impl fmt::Display for EnvelopeProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvelopeProvider::Memory(_) => write!(f, "memory"),
            EnvelopeProvider::SQLite(_) => write!(f, "sqlite"),
        }
    }
}
