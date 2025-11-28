use serde::{Deserialize, Serialize};

use crate::filters::common::ListFilters;

/// Filter parameters for coupon list operations.
///
/// This struct represents the available filters that can be applied when
/// querying coupon lists from the API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CouponFilter {
    // Currently the Lago API does not expose specific filters for coupons
    // beyond pagination. This struct is kept for consistency and future extensibility.
}

impl CouponFilter {
    /// Create a new empty coupon filter.
    ///
    /// # Returns
    /// A new `CouponFilter` instance with no filters set.
    pub fn new() -> Self {
        Self::default()
    }
}

impl ListFilters for CouponFilter {
    /// Converts the coupon filter into HTTP query parameters.
    ///
    /// # Returns
    /// A vector of query parameter tuples containing the filter criteria.
    fn to_query_params(&self) -> Vec<(&str, String)> {
        Vec::new()
    }
}
