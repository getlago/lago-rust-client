//! Optional observability helpers for Lago rate limits.
//!
//! Provides a ready-to-use [`RateLimitInfoCallback`](crate::RateLimitInfoCallback)
//! so callers can opt into rate limit observability without writing their
//! own callback.

use std::sync::Arc;

use crate::client::{RateLimitInfo, RateLimitInfoCallback};

/// Default usage thresholds (80%, 90%, 95%) at which the logging observer
/// emits a warning.
pub const DEFAULT_RATE_LIMIT_THRESHOLDS: &[f64] = &[0.80, 0.90, 0.95];

/// Returns a [`RateLimitInfoCallback`] that logs a warning to stderr each time
/// rate limit usage crosses one of the configured thresholds.
///
/// Pass `None` for `thresholds` to use [`DEFAULT_RATE_LIMIT_THRESHOLDS`].
///
/// # Example
///
/// ```no_run
/// use lago_client::{Config, Credentials, LagoClient, Region, observability::logging_rate_limit_observer};
///
/// let config = Config::builder()
///     .credentials(Credentials::new("api-key".to_string()))
///     .region(Region::Us)
///     .on_rate_limit_info(logging_rate_limit_observer(None))
///     .build();
///
/// let _client = LagoClient::new(config);
/// ```
pub fn logging_rate_limit_observer(thresholds: Option<&[f64]>) -> RateLimitInfoCallback {
    let mut sorted: Vec<f64> = thresholds.unwrap_or(DEFAULT_RATE_LIMIT_THRESHOLDS).to_vec();
    // Descending so we report the highest matching threshold first.
    sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

    Arc::new(move |info: &RateLimitInfo| {
        let Some(pct) = info.usage_pct() else {
            return;
        };

        if sorted.iter().any(|threshold| pct >= *threshold) {
            eprintln!(
                "lago: rate limit at {:.0}% (limit={:?}, remaining={:?}, reset={:?}s, {} {})",
                pct * 100.0,
                info.limit,
                info.remaining,
                info.reset,
                info.method,
                info.url,
            );
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn info(limit: Option<u32>, remaining: Option<u32>) -> RateLimitInfo {
        RateLimitInfo {
            limit,
            remaining,
            reset: Some(10),
            method: "GET".to_string(),
            url: "https://x".to_string(),
        }
    }

    #[test]
    fn observer_does_not_panic_for_valid_info() {
        let observer = logging_rate_limit_observer(Some(&[0.80]));
        observer(&info(Some(100), Some(4))); // 96%
    }

    #[test]
    fn observer_no_op_when_usage_pct_unavailable() {
        let observer = logging_rate_limit_observer(None);
        observer(&info(None, None));
    }

    #[test]
    fn default_thresholds_used_when_none_passed() {
        // Smoke test: should not panic with the default thresholds.
        let observer = logging_rate_limit_observer(None);
        observer(&info(Some(100), Some(50)));
    }
}
