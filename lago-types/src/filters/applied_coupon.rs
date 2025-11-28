use serde::{Deserialize, Serialize};

use crate::filters::common::ListFilters;
use crate::models::AppliedCouponStatus;

/// Filter parameters for applied coupon list operations.
///
/// This struct represents the available filters that can be applied when
/// querying applied coupon lists from the API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppliedCouponFilter {
    pub status: Option<AppliedCouponStatus>,
    pub external_customer_id: Option<String>,
    pub coupon_codes: Option<Vec<String>>,
}

impl AppliedCouponFilter {
    /// Create a new empty applied coupon filter.
    ///
    /// # Returns
    /// A new `AppliedCouponFilter` instance with no filters set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the status filter.
    ///
    /// # Arguments
    /// * `status` - The applied coupon status to filter by (active or terminated).
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_status(mut self, status: AppliedCouponStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the external customer ID filter.
    ///
    /// # Arguments
    /// * `customer_id` - The external customer ID to filter by.
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_external_customer_id(mut self, customer_id: String) -> Self {
        self.external_customer_id = Some(customer_id);
        self
    }

    /// Sets the coupon codes filter.
    ///
    /// # Arguments
    /// * `coupon_codes` - The coupon codes to filter by.
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_coupon_codes(mut self, coupon_codes: Vec<String>) -> Self {
        self.coupon_codes = Some(coupon_codes);
        self
    }
}

impl ListFilters for AppliedCouponFilter {
    /// Converts the applied coupon filter into HTTP query parameters.
    ///
    /// # Returns
    /// A vector of query parameter tuples containing the filter criteria.
    fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params: Vec<(&'static str, String)> = Vec::new();

        if let Some(status) = &self.status {
            let status_str = match status {
                AppliedCouponStatus::Active => "active",
                AppliedCouponStatus::Terminated => "terminated",
            };
            params.push(("status", status_str.to_string()));
        }

        if let Some(customer_id) = &self.external_customer_id {
            params.push(("external_customer_id", customer_id.clone()));
        }

        if let Some(coupon_codes) = &self.coupon_codes {
            for code in coupon_codes {
                params.push(("coupon_code[]", code.clone()));
            }
        }

        params
    }
}
