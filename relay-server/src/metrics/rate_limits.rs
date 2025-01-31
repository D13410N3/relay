//! Quota and rate limiting helpers for metrics and metrics buckets.

use itertools::Either;
use relay_metrics::Bucket;
use relay_quotas::{DataCategory, Quota, RateLimits, Scoping};

use crate::metrics::outcomes::{BucketSummary, MetricOutcomes, TrackableBucket};
use crate::services::outcome::Outcome;
use crate::utils;

/// Contains all data necessary to rate limit metrics or metrics buckets.
#[derive(Debug)]
pub struct MetricsLimiter<Q: AsRef<Vec<Quota>> = Vec<Quota>> {
    /// A list of aggregated metric buckets with some counters.
    buckets: Vec<SummarizedBucket>,

    /// The quotas set on the current project.
    quotas: Q,

    /// Project information.
    scoping: Scoping,

    /// The number of performance items (transactions, spans, profiles) contributing to these metrics.
    #[cfg(feature = "processing")]
    counts: EntityCounts,
}

fn to_counts(summary: &BucketSummary) -> EntityCounts {
    match *summary {
        BucketSummary::Transactions(count) => EntityCounts {
            transactions: Some(count),
            spans: None,
        },
        BucketSummary::Spans(count) => EntityCounts {
            transactions: None,
            spans: Some(count),
        },
        BucketSummary::None => EntityCounts::default(),
    }
}

#[derive(Debug)]
struct SummarizedBucket {
    bucket: Bucket,
    summary: BucketSummary,
}

/// Contains the total counts of limitable entities represented by a batch of metrics.
#[derive(Debug, Default, Clone)]
struct EntityCounts {
    /// The number of transactions represented in the current batch.
    ///
    /// - `None` if the batch does not contain any transaction metrics.
    /// - `Some(0)` if the batch contains transaction metrics, but no `usage` count.
    /// - `Some(n > 0)` if the batch contains a `usage` count for transactions.
    ///
    /// The distinction between `None` and `Some(0)` is needed to decide whether or not a rate limit
    /// must be checked.
    transactions: Option<usize>,
    /// The number of spans represented in the current batch.
    ///
    /// - `None` if the batch does not contain any transaction metrics.
    /// - `Some(0)` if the batch contains transaction metrics, but no `usage` count.
    /// - `Some(n > 0)` if the batch contains a `usage` count for spans.
    ///
    /// The distinction between `None` and `Some(0)` is needed to decide whether or not a rate limit
    /// must be checked.
    spans: Option<usize>,
}

impl std::ops::Add for EntityCounts {
    type Output = Self;

    fn add(self, rhs: EntityCounts) -> Self::Output {
        Self {
            transactions: add_some(self.transactions, rhs.transactions),
            spans: add_some(self.spans, rhs.spans),
        }
    }
}

fn add_some<T>(a: Option<T>, b: Option<T>) -> Option<T>
where
    T: std::ops::Add<Output = T>,
{
    match (a, b) {
        (None, None) => None,
        (None, Some(c)) | (Some(c), None) => Some(c),
        (Some(a), Some(b)) => Some(a + b),
    }
}

impl<Q: AsRef<Vec<Quota>>> MetricsLimiter<Q> {
    /// Create a new limiter instance.
    ///
    /// Returns Ok if `metrics` contain relevant metrics, `metrics` otherwise.
    pub fn create(
        buckets: impl IntoIterator<Item = Bucket>,
        quotas: Q,
        scoping: Scoping,
    ) -> Result<Self, Vec<Bucket>> {
        let buckets: Vec<_> = buckets
            .into_iter()
            .map(|bucket| {
                // Sampled buckets are not rate limited, because the sample has already been rate limited.
                let summary = match bucket.metadata.extracted_from_indexed {
                    false => bucket.summary(),
                    true => Default::default(),
                };
                SummarizedBucket { bucket, summary }
            })
            .collect();

        // Accumulate the total counts
        let total_counts = buckets
            .iter()
            .map(|b| to_counts(&b.summary))
            .reduce(|a, b| a + b);
        if let Some(_counts) = total_counts {
            Ok(Self {
                buckets,
                quotas,
                scoping,
                #[cfg(feature = "processing")]
                counts: _counts,
            })
        } else {
            Err(buckets.into_iter().map(|s| s.bucket).collect())
        }
    }

    /// Returns a reference to the scoping information.
    #[cfg(feature = "processing")]
    pub fn scoping(&self) -> &Scoping {
        &self.scoping
    }

    /// Returns a reference to the list of quotas.
    #[cfg(feature = "processing")]
    pub fn quotas(&self) -> &[Quota] {
        self.quotas.as_ref()
    }

    /// Counts the number of transactions/spans represented in this batch.
    ///
    /// Returns
    /// - `None` if the batch does not contain metrics related to the data category.
    /// - `Some(0)` if the batch contains metrics related to the data category, but no `usage` count.
    /// - `Some(n > 0)` if the batch contains a `usage` count for the given data category.
    ///
    /// The distinction between `None` and `Some(0)` is needed to decide whether or not a rate limit
    /// must be checked.
    #[cfg(feature = "processing")]
    pub fn count(&self, category: DataCategory) -> Option<usize> {
        match category {
            DataCategory::Transaction => self.counts.transactions,
            DataCategory::Span => self.counts.spans,
            _ => None,
        }
    }

    fn drop_with_outcome(
        &mut self,
        category: DataCategory,
        outcome: Outcome,
        metric_outcomes: &MetricOutcomes,
    ) {
        let buckets = std::mem::take(&mut self.buckets);
        let (buckets, dropped) = utils::split_off_map(buckets, |b| match b.summary {
            BucketSummary::Transactions { .. } if category == DataCategory::Transaction => {
                Either::Right(b.bucket)
            }
            BucketSummary::Spans(_) if category == DataCategory::Span => Either::Right(b.bucket),
            _ => Either::Left(b),
        });
        self.buckets = buckets;

        metric_outcomes.track(self.scoping, &dropped, outcome);
    }

    // Drop transaction-related metrics and create outcomes for any active rate limits.
    //
    // Returns true if any metrics were dropped.
    pub fn enforce_limits(
        &mut self,
        rate_limits: &RateLimits,
        metric_outcomes: &MetricOutcomes,
    ) -> bool {
        for category in [DataCategory::Transaction, DataCategory::Span] {
            let active_rate_limits =
                rate_limits.check_with_quotas(self.quotas.as_ref(), self.scoping.item(category));

            // If a rate limit is active, discard relevant buckets.
            if let Some(limit) = active_rate_limits.longest() {
                self.drop_with_outcome(
                    category,
                    Outcome::RateLimited(limit.reason_code.clone()),
                    metric_outcomes,
                );

                return true;
            }
        }

        false
    }

    /// Consume this struct and return the contained metrics.
    pub fn into_buckets(self) -> Vec<Bucket> {
        self.buckets.into_iter().map(|s| s.bucket).collect()
    }
}

#[cfg(test)]
mod tests {
    use relay_base_schema::organization::OrganizationId;
    use relay_base_schema::project::{ProjectId, ProjectKey};
    use relay_metrics::{BucketMetadata, BucketValue, UnixTimestamp};
    use relay_quotas::QuotaScope;
    use relay_system::Addr;
    use smallvec::smallvec;

    use crate::metrics::MetricStats;

    use super::*;

    fn deny(category: DataCategory) -> Vec<Quota> {
        vec![Quota {
            id: None,
            categories: smallvec![category],
            scope: QuotaScope::Organization,
            scope_id: None,
            limit: Some(0),
            window: None,
            reason_code: None,
            namespace: None,
        }]
    }

    /// Applies rate limits and returns the remaining buckets and generated outcomes.
    fn run_limiter(
        metrics: Vec<Bucket>,
        quotas: Vec<Quota>,
    ) -> (Vec<Bucket>, Vec<(Outcome, DataCategory, u32)>) {
        let (outcome_sink, mut rx) = Addr::custom();
        let metric_outcomes = MetricOutcomes::new(MetricStats::test().0, outcome_sink.clone());

        let mut limiter = MetricsLimiter::create(
            metrics,
            quotas,
            Scoping {
                organization_id: OrganizationId::new(1),
                project_id: ProjectId::new(1),
                project_key: ProjectKey::parse("a94ae32be2584e0bbd7a4cbb95971fee").unwrap(),
                key_id: None,
            },
        )
        .unwrap();

        limiter.enforce_limits(&RateLimits::new(), &metric_outcomes);
        let metrics = limiter.into_buckets();

        rx.close();

        let outcomes: Vec<_> = (0..)
            .map(|_| rx.blocking_recv())
            .take_while(|o| o.is_some())
            .flatten()
            .filter(|o| o.category != DataCategory::MetricBucket)
            .map(|o| (o.outcome, o.category, o.quantity))
            .collect();

        (metrics, outcomes)
    }

    /// A few different bucket types
    fn mixed_bag() -> Vec<Bucket> {
        vec![
            Bucket {
                timestamp: UnixTimestamp::now(),
                width: 0,
                name: "c:transactions/usage@none".into(),
                tags: Default::default(),
                value: BucketValue::counter(12.into()),
                metadata: BucketMetadata::default(),
            },
            Bucket {
                timestamp: UnixTimestamp::now(),
                width: 0,
                name: "c:spans/usage@none".into(),
                tags: Default::default(),
                value: BucketValue::counter(34.into()),
                metadata: BucketMetadata::default(),
            },
            Bucket {
                timestamp: UnixTimestamp::now(),
                width: 0,
                name: "c:spans/usage@none".into(),
                tags: Default::default(),
                value: BucketValue::counter(56.into()),
                metadata: BucketMetadata::default(),
            },
            Bucket {
                timestamp: UnixTimestamp::now(),
                width: 0,
                name: "d:spans/exclusive_time@millisecond".into(),
                tags: Default::default(),
                value: BucketValue::distribution(78.into()),
                metadata: BucketMetadata::default(),
            },
            Bucket {
                timestamp: UnixTimestamp::now(),
                width: 0,
                name: "d:custom/something@millisecond".into(),
                tags: Default::default(),
                value: BucketValue::distribution(78.into()),
                metadata: BucketMetadata::default(),
            },
        ]
    }

    #[test]
    fn mixed_with_span_quota() {
        let (metrics, outcomes) = run_limiter(mixed_bag(), deny(DataCategory::Span));

        assert_eq!(metrics.len(), 2);
        assert_eq!(&*metrics[0].name, "c:transactions/usage@none");
        assert_eq!(&*metrics[1].name, "d:custom/something@millisecond");

        assert_eq!(
            outcomes,
            vec![(Outcome::RateLimited(None), DataCategory::Span, 90)]
        );
    }

    #[test]
    fn mixed_with_transaction_quota() {
        let (metrics, outcomes) = run_limiter(mixed_bag(), deny(DataCategory::Transaction));

        assert_eq!(metrics.len(), 4);
        assert_eq!(&*metrics[0].name, "c:spans/usage@none");
        assert_eq!(&*metrics[1].name, "c:spans/usage@none");
        assert_eq!(&*metrics[2].name, "d:spans/exclusive_time@millisecond");
        assert_eq!(&*metrics[3].name, "d:custom/something@millisecond");

        assert_eq!(
            outcomes,
            vec![(Outcome::RateLimited(None), DataCategory::Transaction, 12)]
        );
    }
}
