use std::fmt;
use std::str::FromStr;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use relay_base_schema::metrics::MetricNamespace;
use relay_base_schema::organization::OrganizationId;
use relay_base_schema::project::{ProjectId, ProjectKey};
use smallvec::SmallVec;

use crate::quota::{DataCategories, ItemScoping, Quota, QuotaScope, ReasonCode, Scoping};
use crate::REJECT_ALL_SECS;

/// A monotonic expiration marker for `RateLimit`s.
///
/// `RetryAfter` marks an instant at which a rate limit expires, which is indicated by `expired`. It
/// can convert into the remaining time until expiration.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RetryAfter {
    when: Instant,
}

impl RetryAfter {
    /// Creates a retry after instance.
    #[inline]
    pub fn from_secs(seconds: u64) -> Self {
        let now = Instant::now();
        let when = now.checked_add(Duration::from_secs(seconds)).unwrap_or(now);
        Self { when }
    }

    /// Returns the remaining duration until the rate limit expires using the passed instant as the
    /// reference point.
    #[inline]
    pub fn remaining_at(self, at: Instant) -> Option<Duration> {
        if at >= self.when {
            None
        } else {
            Some(self.when - at)
        }
    }

    /// Returns the remaining duration until the rate limit expires.
    #[inline]
    pub fn remaining(self) -> Option<Duration> {
        self.remaining_at(Instant::now())
    }

    /// Returns the remaining seconds until the rate limit expires using the passed instant as the
    /// reference point.
    ///
    /// This is a shortcut to `retry_after.remaining().as_secs()` with one exception: If the rate
    /// limit has expired, this function returns `0`.
    #[inline]
    pub fn remaining_seconds_at(self, at: Instant) -> u64 {
        match self.remaining_at(at) {
            // Compensate for the missing subsec part by adding 1s
            Some(duration) if duration.subsec_nanos() == 0 => duration.as_secs(),
            Some(duration) => duration.as_secs() + 1,
            None => 0,
        }
    }

    /// Returns the remaining seconds until the rate limit expires.
    #[inline]
    pub fn remaining_seconds(self) -> u64 {
        self.remaining_seconds_at(Instant::now())
    }

    /// Returns whether this rate limit has expired at the passed instant.
    #[inline]
    pub fn expired_at(self, at: Instant) -> bool {
        self.remaining_at(at).is_none()
    }

    /// Returns whether this rate limit has expired.
    #[inline]
    pub fn expired(self) -> bool {
        self.remaining_at(Instant::now()).is_none()
    }
}

impl fmt::Debug for RetryAfter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.remaining_seconds() {
            0 => write!(f, "RetryAfter(expired)"),
            remaining => write!(f, "RetryAfter({remaining}s)"),
        }
    }
}

#[cfg(test)]
impl serde::Serialize for RetryAfter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeTupleStruct;
        let mut tup = serializer.serialize_tuple_struct("RetryAfter", 1)?;
        tup.serialize_field(&self.remaining_seconds())?;
        tup.end()
    }
}

#[derive(Debug)]
/// Error parsing a `RetryAfter`.
pub enum InvalidRetryAfter {
    /// The supplied delay in seconds was not valid.
    InvalidDelay(std::num::ParseFloatError),
}

impl FromStr for RetryAfter {
    type Err = InvalidRetryAfter;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let float = s.parse::<f64>().map_err(InvalidRetryAfter::InvalidDelay)?;
        let seconds = float.max(0.0).ceil() as u64;
        Ok(RetryAfter::from_secs(seconds))
    }
}

/// The scope that a rate limit applied to.
///
/// As opposed to `QuotaScope`, which only declared the class of the scope, this also carries
/// information about the scope instance. That is, the specific identifiers of the individual scopes
/// that a rate limit applied to.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(serde::Serialize))]
pub enum RateLimitScope {
    /// Global scope.
    Global,
    /// An organization with identifier.
    Organization(OrganizationId),
    /// A project with identifier.
    Project(ProjectId),
    /// A DSN public key.
    Key(ProjectKey),
}

impl RateLimitScope {
    /// Extracts a rate limiting scope from the given item scoping for a specific quota.
    pub fn for_quota(scoping: &Scoping, scope: QuotaScope) -> Self {
        match scope {
            QuotaScope::Global => Self::Global,
            QuotaScope::Organization => Self::Organization(scoping.organization_id),
            QuotaScope::Project => Self::Project(scoping.project_id),
            QuotaScope::Key => Self::Key(scoping.project_key),
            // For unknown scopes, assume the most specific scope:
            QuotaScope::Unknown => Self::Key(scoping.project_key),
        }
    }

    /// Returns the canonical name of this scope.
    pub fn name(&self) -> &'static str {
        match *self {
            Self::Global => QuotaScope::Global.name(),
            Self::Key(_) => QuotaScope::Key.name(),
            Self::Project(_) => QuotaScope::Project.name(),
            Self::Organization(_) => QuotaScope::Organization.name(),
        }
    }
}

/// A bounded rate limit.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct RateLimit {
    /// A set of data categories that this quota applies to. If missing or empty, this rate limit
    /// applies to all data.
    pub categories: DataCategories,

    /// The scope of this rate limit.
    pub scope: RateLimitScope,

    /// A machine readable reason indicating which quota caused it.
    pub reason_code: Option<ReasonCode>,

    /// A marker when this rate limit expires.
    pub retry_after: RetryAfter,

    /// The metric namespace of this rate limit.
    ///
    /// Ignored on all data categories except for `MetricBucket`. If empty, this rate limit applies
    /// to metrics of all namespaces.
    pub namespaces: SmallVec<[MetricNamespace; 1]>,
}

impl RateLimit {
    /// Creates a new rate limit for the given `Quota`.
    pub fn from_quota(quota: &Quota, scoping: &Scoping, retry_after: RetryAfter) -> Self {
        Self {
            categories: quota.categories.clone(),
            scope: RateLimitScope::for_quota(scoping, quota.scope),
            reason_code: quota.reason_code.clone(),
            retry_after,
            namespaces: quota.namespace.into_iter().collect(),
        }
    }

    /// Checks whether the rate limit applies to the given item.
    pub fn matches(&self, scoping: ItemScoping<'_>) -> bool {
        self.matches_scope(scoping)
            && scoping.matches_categories(&self.categories)
            && scoping.matches_namespaces(&self.namespaces)
    }

    /// Returns `true` if the rate limiting scope matches the given item.
    fn matches_scope(&self, scoping: ItemScoping<'_>) -> bool {
        match self.scope {
            RateLimitScope::Global => true,
            RateLimitScope::Organization(org_id) => scoping.organization_id == org_id,
            RateLimitScope::Project(project_id) => scoping.project_id == project_id,
            RateLimitScope::Key(ref key) => scoping.project_key == *key,
        }
    }
}

/// A collection of scoped rate limits.
///
/// This collection may be empty, indicated by `is_ok`. If this instance carries rate limits, they
/// can be iterated over using `iter`. Additionally, rate limits can be checked for items by
/// invoking `check` with the respective `ItemScoping`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct RateLimits {
    limits: Vec<RateLimit>,
}

impl RateLimits {
    /// Creates an empty RateLimits instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a limit to this collection.
    ///
    /// If a rate limit with an overlapping scope already exists, the `retry_after` count is merged
    /// with the existing limit. Otherwise, the new rate limit is added.
    pub fn add(&mut self, mut limit: RateLimit) {
        // Categories are logically a set, but not implemented as such.
        limit.categories.sort();

        let limit_opt = self.limits.iter_mut().find(|l| {
            let RateLimit {
                categories,
                scope,
                reason_code: _,
                retry_after: _,
                namespaces: namespace,
            } = &limit;

            *categories == l.categories && *scope == l.scope && *namespace == l.namespaces
        });

        match limit_opt {
            None => self.limits.push(limit),
            Some(existing) if existing.retry_after < limit.retry_after => *existing = limit,
            Some(_) => (), // keep existing, longer limit
        }
    }

    /// Merges all limits into this instance.
    ///
    /// This keeps all existing rate limits, adding new ones, and updating ones with a longer
    /// `retry_after` count. The resulting `RateLimits` contains the merged maximum.
    pub fn merge(&mut self, limits: Self) {
        for limit in limits {
            self.add(limit);
        }
    }

    /// Returns `true` if this instance contains no active limits.
    pub fn is_ok(&self) -> bool {
        !self.is_limited()
    }

    /// Returns `true` if this instance contains active rate limits.
    pub fn is_limited(&self) -> bool {
        let now = Instant::now();
        self.iter().any(|limit| !limit.retry_after.expired_at(now))
    }

    /// Removes expired rate limits from this instance.
    pub fn clean_expired(&mut self, now: Instant) {
        self.limits
            .retain(|limit| !limit.retry_after.expired_at(now));
    }

    /// Checks whether any rate limits apply to the given scoping.
    ///
    /// If no limits match, then the returned `RateLimits` instance evaluates `is_ok`. Otherwise, it
    /// contains rate limits that match the given scoping.
    pub fn check(&self, scoping: ItemScoping<'_>) -> Self {
        self.check_with_quotas(&[], scoping)
    }

    /// Checks whether any rate limits apply to the given scoping.
    ///
    /// This is similar to `check`. Additionally, it checks for quotas with a static limit `0`, and
    /// rejects items even if there is no active rate limit in this instance.
    ///
    /// If no limits or quotas match, then the returned `RateLimits` instance evaluates `is_ok`.
    /// Otherwise, it contains rate limits that match the given scoping.
    pub fn check_with_quotas<'a>(
        &self,
        quotas: impl IntoIterator<Item = &'a Quota>,
        scoping: ItemScoping<'_>,
    ) -> Self {
        let mut applied_limits = Self::new();

        for quota in quotas {
            if quota.limit == Some(0) && quota.matches(scoping) {
                let retry_after = RetryAfter::from_secs(REJECT_ALL_SECS);
                applied_limits.add(RateLimit::from_quota(quota, &scoping, retry_after));
            }
        }

        for limit in &self.limits {
            if limit.matches(scoping) {
                applied_limits.add(limit.clone());
            }
        }

        applied_limits
    }

    /// Returns an iterator over the rate limits.
    pub fn iter(&self) -> RateLimitsIter<'_> {
        RateLimitsIter {
            iter: self.limits.iter(),
        }
    }

    /// Returns the longest rate limit.
    ///
    /// If multiple rate limits have the same retry after count, any of the limits is returned.
    pub fn longest(&self) -> Option<&RateLimit> {
        self.iter().max_by_key(|limit| limit.retry_after)
    }

    /// Returns `true` if there are any limits contained.
    ///
    /// This is equivalent to checking whether [`Self::longest`] returns `Some`
    /// or [`Self::iter`] returns an iterator with at least one item.
    pub fn is_empty(&self) -> bool {
        self.limits.is_empty()
    }
}

/// Immutable rate limits iterator.
///
/// This struct is created by the `iter` method on `RateLimits`.
pub struct RateLimitsIter<'a> {
    iter: std::slice::Iter<'a, RateLimit>,
}

impl<'a> Iterator for RateLimitsIter<'a> {
    type Item = &'a RateLimit;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl IntoIterator for RateLimits {
    type IntoIter = RateLimitsIntoIter;
    type Item = RateLimit;

    fn into_iter(self) -> Self::IntoIter {
        RateLimitsIntoIter {
            iter: self.limits.into_iter(),
        }
    }
}

/// An iterator that moves out of `RateLimtis`.
///
/// This struct is created by the `into_iter` method on `RateLimits`, provided by the `IntoIterator`
/// trait.
pub struct RateLimitsIntoIter {
    iter: std::vec::IntoIter<RateLimit>,
}

impl Iterator for RateLimitsIntoIter {
    type Item = RateLimit;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> IntoIterator for &'a RateLimits {
    type IntoIter = RateLimitsIter<'a>;
    type Item = &'a RateLimit;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Like [`RateLimits`], a collection of scoped rate limits but with all the checks
/// necessary to cache the limits.
///
/// The data structure makes sure no expired rate limits are enforced.
#[derive(Debug, Default)]
pub struct CachedRateLimits(Mutex<Arc<RateLimits>>);

impl CachedRateLimits {
    /// Creates a new, empty instance without any rate limits enforced.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a limit to this collection.
    ///
    /// See also: [`RateLimits::add`].
    pub fn add(&self, limit: RateLimit) {
        let mut inner = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        let current = Arc::make_mut(&mut inner);
        current.add(limit);
    }

    /// Merges more rate limits into this instance.
    ///
    /// See also: [`RateLimits::merge`].
    pub fn merge(&self, limits: RateLimits) {
        let mut inner = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        let current = Arc::make_mut(&mut inner);
        for limit in limits {
            current.add(limit)
        }
    }

    /// Returns a reference to the contained rate limits.
    ///
    /// This call guarantees that at the time of call no returned rate limit is expired.
    pub fn current_limits(&self) -> Arc<RateLimits> {
        let now = Instant::now();
        let mut inner = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        Arc::make_mut(&mut inner).clean_expired(now);
        Arc::clone(&inner)
    }
}

#[cfg(test)]
mod tests {
    use smallvec::smallvec;

    use super::*;
    use crate::quota::DataCategory;
    use crate::MetricNamespaceScoping;

    #[test]
    fn test_parse_retry_after() {
        // positive float always rounds up to the next integer
        let retry_after = "17.1".parse::<RetryAfter>().expect("parse RetryAfter");
        assert_eq!(retry_after.remaining_seconds(), 18);
        assert!(!retry_after.expired());
        let retry_after = "17.7".parse::<RetryAfter>().expect("parse RetryAfter");
        assert_eq!(retry_after.remaining_seconds(), 18);
        assert!(!retry_after.expired());

        // positive int
        let retry_after = "17".parse::<RetryAfter>().expect("parse RetryAfter");
        assert_eq!(retry_after.remaining_seconds(), 17);
        assert!(!retry_after.expired());

        // negative numbers are treated as zero
        let retry_after = "-2".parse::<RetryAfter>().expect("parse RetryAfter");
        assert_eq!(retry_after.remaining_seconds(), 0);
        assert!(retry_after.expired());
        let retry_after = "-inf".parse::<RetryAfter>().expect("parse RetryAfter");
        assert_eq!(retry_after.remaining_seconds(), 0);
        assert!(retry_after.expired());

        // inf and NaN are valid input and treated as zero
        let retry_after = "inf".parse::<RetryAfter>().expect("parse RetryAfter");
        assert_eq!(retry_after.remaining_seconds(), 0);
        assert!(retry_after.expired());
        let retry_after = "NaN".parse::<RetryAfter>().expect("parse RetryAfter");
        assert_eq!(retry_after.remaining_seconds(), 0);
        assert!(retry_after.expired());

        // large inputs that would overflow are treated as zero
        let retry_after = "100000000000000000000"
            .parse::<RetryAfter>()
            .expect("parse RetryAfter");
        assert_eq!(retry_after.remaining_seconds(), 0);
        assert!(retry_after.expired());

        // invalid strings cause parse error
        "".parse::<RetryAfter>().expect_err("error RetryAfter");
        "nope".parse::<RetryAfter>().expect_err("error RetryAfter");
        " 2 ".parse::<RetryAfter>().expect_err("error RetryAfter");
        "6 0".parse::<RetryAfter>().expect_err("error RetryAfter");
    }

    #[test]
    fn test_rate_limit_matches_categories() {
        let rate_limit = RateLimit {
            categories: smallvec![DataCategory::Unknown, DataCategory::Error],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: None,
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![],
        };

        assert!(rate_limit.matches(ItemScoping {
            category: DataCategory::Error,
            scoping: &Scoping {
                organization_id: OrganizationId::new(42),
                project_id: ProjectId::new(21),
                project_key: ProjectKey::parse("a94ae32be2584e0bbd7a4cbb95971fee").unwrap(),
                key_id: None,
            },
            namespace: MetricNamespaceScoping::None,
        }));

        assert!(!rate_limit.matches(ItemScoping {
            category: DataCategory::Transaction,
            scoping: &Scoping {
                organization_id: OrganizationId::new(42),
                project_id: ProjectId::new(21),
                project_key: ProjectKey::parse("a94ae32be2584e0bbd7a4cbb95971fee").unwrap(),
                key_id: None,
            },
            namespace: MetricNamespaceScoping::None,
        }));
    }

    #[test]
    fn test_rate_limit_matches_organization() {
        let rate_limit = RateLimit {
            categories: DataCategories::new(),
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: None,
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![],
        };

        assert!(rate_limit.matches(ItemScoping {
            category: DataCategory::Error,
            scoping: &Scoping {
                organization_id: OrganizationId::new(42),
                project_id: ProjectId::new(21),
                project_key: ProjectKey::parse("a94ae32be2584e0bbd7a4cbb95971fee").unwrap(),
                key_id: None,
            },
            namespace: MetricNamespaceScoping::None,
        }));

        assert!(!rate_limit.matches(ItemScoping {
            category: DataCategory::Error,
            scoping: &Scoping {
                organization_id: OrganizationId::new(0),
                project_id: ProjectId::new(21),
                project_key: ProjectKey::parse("a94ae32be2584e0bbd7a4cbb95971fee").unwrap(),
                key_id: None,
            },
            namespace: MetricNamespaceScoping::None,
        }));
    }

    #[test]
    fn test_rate_limit_matches_project() {
        let rate_limit = RateLimit {
            categories: DataCategories::new(),
            scope: RateLimitScope::Project(ProjectId::new(21)),
            reason_code: None,
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![],
        };

        assert!(rate_limit.matches(ItemScoping {
            category: DataCategory::Error,
            scoping: &Scoping {
                organization_id: OrganizationId::new(42),
                project_id: ProjectId::new(21),
                project_key: ProjectKey::parse("a94ae32be2584e0bbd7a4cbb95971fee").unwrap(),
                key_id: None,
            },
            namespace: MetricNamespaceScoping::None,
        }));

        assert!(!rate_limit.matches(ItemScoping {
            category: DataCategory::Error,
            scoping: &Scoping {
                organization_id: OrganizationId::new(42),
                project_id: ProjectId::new(0),
                project_key: ProjectKey::parse("a94ae32be2584e0bbd7a4cbb95971fee").unwrap(),
                key_id: None,
            },
            namespace: MetricNamespaceScoping::None,
        }));
    }

    #[test]
    fn test_rate_limit_matches_namespaces() {
        let rate_limit = RateLimit {
            categories: smallvec![],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: None,
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![MetricNamespace::Custom],
        };

        let scoping = Scoping {
            organization_id: OrganizationId::new(42),
            project_id: ProjectId::new(21),
            project_key: ProjectKey::parse("a94ae32be2584e0bbd7a4cbb95971fee").unwrap(),
            key_id: None,
        };

        assert!(rate_limit.matches(ItemScoping {
            category: DataCategory::MetricBucket,
            scoping: &scoping,
            namespace: MetricNamespaceScoping::Some(MetricNamespace::Custom),
        }));

        assert!(!rate_limit.matches(ItemScoping {
            category: DataCategory::MetricBucket,
            scoping: &scoping,
            namespace: MetricNamespaceScoping::Some(MetricNamespace::Spans),
        }));

        let general_rate_limit = RateLimit {
            categories: smallvec![],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: None,
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![], // all namespaces
        };

        assert!(general_rate_limit.matches(ItemScoping {
            category: DataCategory::MetricBucket,
            scoping: &scoping,
            namespace: MetricNamespaceScoping::Some(MetricNamespace::Spans),
        }));

        assert!(general_rate_limit.matches(ItemScoping {
            category: DataCategory::MetricBucket,
            scoping: &scoping,
            namespace: MetricNamespaceScoping::None,
        }));
    }

    #[test]
    fn test_rate_limit_matches_key() {
        let rate_limit = RateLimit {
            categories: DataCategories::new(),
            scope: RateLimitScope::Key(
                ProjectKey::parse("a94ae32be2584e0bbd7a4cbb95971fee").unwrap(),
            ),
            reason_code: None,
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![],
        };

        assert!(rate_limit.matches(ItemScoping {
            category: DataCategory::Error,
            scoping: &Scoping {
                organization_id: OrganizationId::new(42),
                project_id: ProjectId::new(21),
                project_key: ProjectKey::parse("a94ae32be2584e0bbd7a4cbb95971fee").unwrap(),
                key_id: None,
            },
            namespace: MetricNamespaceScoping::None,
        }));

        assert!(!rate_limit.matches(ItemScoping {
            category: DataCategory::Error,
            scoping: &Scoping {
                organization_id: OrganizationId::new(0),
                project_id: ProjectId::new(21),
                project_key: ProjectKey::parse("deadbeefdeadbeefdeadbeefdeadbeef").unwrap(),
                key_id: None,
            },
            namespace: MetricNamespaceScoping::None,
        }));
    }

    #[test]
    fn test_rate_limits_add_replacement() {
        let mut rate_limits = RateLimits::new();

        rate_limits.add(RateLimit {
            categories: smallvec![DataCategory::Default, DataCategory::Error],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: Some(ReasonCode::new("first")),
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![],
        });

        // longer rate limit shadows shorter one
        rate_limits.add(RateLimit {
            categories: smallvec![DataCategory::Error, DataCategory::Default],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: Some(ReasonCode::new("second")),
            retry_after: RetryAfter::from_secs(10),
            namespaces: smallvec![],
        });

        insta::assert_ron_snapshot!(rate_limits, @r###"
        RateLimits(
          limits: [
            RateLimit(
              categories: [
                default,
                error,
              ],
              scope: Organization(OrganizationId(42)),
              reason_code: Some(ReasonCode("second")),
              retry_after: RetryAfter(10),
              namespaces: [],
            ),
          ],
        )
        "###);
    }

    #[test]
    fn test_rate_limits_add_shadowing() {
        let mut rate_limits = RateLimits::new();

        rate_limits.add(RateLimit {
            categories: smallvec![DataCategory::Default, DataCategory::Error],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: Some(ReasonCode::new("first")),
            retry_after: RetryAfter::from_secs(10),
            namespaces: smallvec![],
        });

        // shorter rate limit is shadowed by existing one
        rate_limits.add(RateLimit {
            categories: smallvec![DataCategory::Error, DataCategory::Default],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: Some(ReasonCode::new("second")),
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![],
        });

        insta::assert_ron_snapshot!(rate_limits, @r###"
        RateLimits(
          limits: [
            RateLimit(
              categories: [
                default,
                error,
              ],
              scope: Organization(OrganizationId(42)),
              reason_code: Some(ReasonCode("first")),
              retry_after: RetryAfter(10),
              namespaces: [],
            ),
          ],
        )
        "###);
    }

    #[test]
    fn test_rate_limits_add_buckets() {
        let mut rate_limits = RateLimits::new();

        rate_limits.add(RateLimit {
            categories: smallvec![DataCategory::Error],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: None,
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![],
        });

        // Same scope but different categories
        rate_limits.add(RateLimit {
            categories: smallvec![DataCategory::Transaction],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: None,
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![],
        });

        // Same categories but different scope
        rate_limits.add(RateLimit {
            categories: smallvec![DataCategory::Error],
            scope: RateLimitScope::Project(ProjectId::new(21)),
            reason_code: None,
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![],
        });

        insta::assert_ron_snapshot!(rate_limits, @r###"
        RateLimits(
          limits: [
            RateLimit(
              categories: [
                error,
              ],
              scope: Organization(OrganizationId(42)),
              reason_code: None,
              retry_after: RetryAfter(1),
              namespaces: [],
            ),
            RateLimit(
              categories: [
                transaction,
              ],
              scope: Organization(OrganizationId(42)),
              reason_code: None,
              retry_after: RetryAfter(1),
              namespaces: [],
            ),
            RateLimit(
              categories: [
                error,
              ],
              scope: Project(ProjectId(21)),
              reason_code: None,
              retry_after: RetryAfter(1),
              namespaces: [],
            ),
          ],
        )
        "###);
    }

    /// Regression test that ensures namespaces are correctly added to rate limits.
    #[test]
    fn test_rate_limits_add_namespaces() {
        let mut rate_limits = RateLimits::new();

        rate_limits.add(RateLimit {
            categories: smallvec![DataCategory::MetricBucket],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: None,
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![MetricNamespace::Custom],
        });

        // Same category but different namespaces
        rate_limits.add(RateLimit {
            categories: smallvec![DataCategory::MetricBucket],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: None,
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![MetricNamespace::Spans],
        });

        insta::assert_ron_snapshot!(rate_limits, @r###"
        RateLimits(
          limits: [
            RateLimit(
              categories: [
                metric_bucket,
              ],
              scope: Organization(OrganizationId(42)),
              reason_code: None,
              retry_after: RetryAfter(1),
              namespaces: [
                "custom",
              ],
            ),
            RateLimit(
              categories: [
                metric_bucket,
              ],
              scope: Organization(OrganizationId(42)),
              reason_code: None,
              retry_after: RetryAfter(1),
              namespaces: [
                "spans",
              ],
            ),
          ],
        )
        "###);
    }

    #[test]
    fn test_rate_limits_longest() {
        let mut rate_limits = RateLimits::new();

        rate_limits.add(RateLimit {
            categories: smallvec![DataCategory::Error],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: Some(ReasonCode::new("first")),
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![],
        });

        // Distinct scope to prevent deduplication
        rate_limits.add(RateLimit {
            categories: smallvec![DataCategory::Transaction],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: Some(ReasonCode::new("second")),
            retry_after: RetryAfter::from_secs(10),
            namespaces: smallvec![],
        });

        let rate_limit = rate_limits.longest().unwrap();
        insta::assert_ron_snapshot!(rate_limit, @r###"
        RateLimit(
          categories: [
            transaction,
          ],
          scope: Organization(OrganizationId(42)),
          reason_code: Some(ReasonCode("second")),
          retry_after: RetryAfter(10),
          namespaces: [],
        )
        "###);
    }

    #[test]
    fn test_rate_limits_clean_expired() {
        let mut rate_limits = RateLimits::new();

        // Active error limit
        rate_limits.add(RateLimit {
            categories: smallvec![DataCategory::Error],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: None,
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![],
        });

        // Inactive error limit with distinct scope
        rate_limits.add(RateLimit {
            categories: smallvec![DataCategory::Error],
            scope: RateLimitScope::Project(ProjectId::new(21)),
            reason_code: None,
            retry_after: RetryAfter::from_secs(0),
            namespaces: smallvec![],
        });

        // Sanity check before running `clean_expired`
        assert_eq!(rate_limits.iter().count(), 2);

        rate_limits.clean_expired(Instant::now());

        // Check that the expired limit has been removed
        insta::assert_ron_snapshot!(rate_limits, @r###"
        RateLimits(
          limits: [
            RateLimit(
              categories: [
                error,
              ],
              scope: Organization(OrganizationId(42)),
              reason_code: None,
              retry_after: RetryAfter(1),
              namespaces: [],
            ),
          ],
        )
        "###);
    }

    #[test]
    fn test_rate_limits_check() {
        let mut rate_limits = RateLimits::new();

        // Active error limit
        rate_limits.add(RateLimit {
            categories: smallvec![DataCategory::Error],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: None,
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![],
        });

        // Active transaction limit
        rate_limits.add(RateLimit {
            categories: smallvec![DataCategory::Transaction],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: None,
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![],
        });

        let applied_limits = rate_limits.check(ItemScoping {
            category: DataCategory::Error,
            scoping: &Scoping {
                organization_id: OrganizationId::new(42),
                project_id: ProjectId::new(21),
                project_key: ProjectKey::parse("a94ae32be2584e0bbd7a4cbb95971fee").unwrap(),
                key_id: None,
            },
            namespace: MetricNamespaceScoping::None,
        });

        // Check that the error limit is applied
        insta::assert_ron_snapshot!(applied_limits, @r###"
        RateLimits(
          limits: [
            RateLimit(
              categories: [
                error,
              ],
              scope: Organization(OrganizationId(42)),
              reason_code: None,
              retry_after: RetryAfter(1),
              namespaces: [],
            ),
          ],
        )
        "###);
    }

    #[test]
    fn test_rate_limits_check_quotas() {
        let mut rate_limits = RateLimits::new();

        // Active error limit
        rate_limits.add(RateLimit {
            categories: smallvec![DataCategory::Error],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: None,
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![],
        });

        // Active transaction limit
        rate_limits.add(RateLimit {
            categories: smallvec![DataCategory::Transaction],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: None,
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![],
        });

        let item_scoping = ItemScoping {
            category: DataCategory::Error,
            scoping: &Scoping {
                organization_id: OrganizationId::new(42),
                project_id: ProjectId::new(21),
                project_key: ProjectKey::parse("a94ae32be2584e0bbd7a4cbb95971fee").unwrap(),
                key_id: None,
            },
            namespace: MetricNamespaceScoping::None,
        };

        let quotas = &[Quota {
            id: None,
            categories: smallvec![DataCategory::Error],
            scope: QuotaScope::Organization,
            scope_id: Some("42".to_owned()),
            limit: Some(0),
            window: None,
            reason_code: Some(ReasonCode::new("zero")),
            namespace: None,
        }];

        let applied_limits = rate_limits.check_with_quotas(quotas, item_scoping);

        insta::assert_ron_snapshot!(applied_limits, @r###"
        RateLimits(
          limits: [
            RateLimit(
              categories: [
                error,
              ],
              scope: Organization(OrganizationId(42)),
              reason_code: Some(ReasonCode("zero")),
              retry_after: RetryAfter(60),
              namespaces: [],
            ),
          ],
        )
        "###);
    }

    #[test]
    fn test_rate_limits_merge() {
        let mut rate_limits1 = RateLimits::new();
        let mut rate_limits2 = RateLimits::new();

        rate_limits1.add(RateLimit {
            categories: smallvec![DataCategory::Error],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: Some(ReasonCode::new("first")),
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![],
        });

        rate_limits1.add(RateLimit {
            categories: smallvec![DataCategory::TransactionIndexed],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: None,
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![],
        });

        rate_limits2.add(RateLimit {
            categories: smallvec![DataCategory::Error],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: Some(ReasonCode::new("second")),
            retry_after: RetryAfter::from_secs(10),
            namespaces: smallvec![],
        });

        rate_limits1.merge(rate_limits2);

        insta::assert_ron_snapshot!(rate_limits1, @r###"
        RateLimits(
          limits: [
            RateLimit(
              categories: [
                error,
              ],
              scope: Organization(OrganizationId(42)),
              reason_code: Some(ReasonCode("second")),
              retry_after: RetryAfter(10),
              namespaces: [],
            ),
            RateLimit(
              categories: [
                transaction_indexed,
              ],
              scope: Organization(OrganizationId(42)),
              reason_code: None,
              retry_after: RetryAfter(1),
              namespaces: [],
            ),
          ],
        )
        "###);
    }

    #[test]
    fn test_cached_rate_limits_expired() {
        let cached = CachedRateLimits::new();

        // Active error limit
        cached.add(RateLimit {
            categories: smallvec![DataCategory::Error],
            scope: RateLimitScope::Organization(OrganizationId::new(42)),
            reason_code: None,
            retry_after: RetryAfter::from_secs(1),
            namespaces: smallvec![],
        });

        // Inactive error limit with distinct scope
        cached.add(RateLimit {
            categories: smallvec![DataCategory::Error],
            scope: RateLimitScope::Project(ProjectId::new(21)),
            reason_code: None,
            retry_after: RetryAfter::from_secs(0),
            namespaces: smallvec![],
        });

        let rate_limits = cached.current_limits();

        insta::assert_ron_snapshot!(rate_limits, @r###"
        RateLimits(
          limits: [
            RateLimit(
              categories: [
                error,
              ],
              scope: Organization(OrganizationId(42)),
              reason_code: None,
              retry_after: RetryAfter(1),
              namespaces: [],
            ),
          ],
        )
        "###);
    }
}
