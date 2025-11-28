use serde::{Deserialize, Serialize};

use crate::filters::{applied_coupon::AppliedCouponFilter, common::ListFilters};
use crate::models::{AppliedCouponFrequency, PaginationParams};

/// Request parameters for listing applied coupons.
///
/// This struct combines pagination parameters and applied coupon-specific filters
/// to build a comprehensive request for retrieving applied coupon lists.
#[derive(Debug, Clone)]
pub struct ListAppliedCouponsRequest {
    pub pagination: PaginationParams,
    pub filters: AppliedCouponFilter,
}

impl ListAppliedCouponsRequest {
    /// Creates a new empty list applied coupons request.
    ///
    /// # Returns
    /// A new `ListAppliedCouponsRequest` instance with default pagination and no filters.
    pub fn new() -> Self {
        Self {
            pagination: PaginationParams::default(),
            filters: AppliedCouponFilter::default(),
        }
    }

    /// Sets the pagination parameters for the request.
    ///
    /// # Arguments
    /// * `pagination` - The pagination parameters to use
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_pagination(mut self, pagination: PaginationParams) -> Self {
        self.pagination = pagination;
        self
    }

    /// Sets the applied coupon filters for the request.
    ///
    /// # Arguments
    /// * `filters` - The applied coupon filters to apply
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_filters(mut self, filters: AppliedCouponFilter) -> Self {
        self.filters = filters;
        self
    }

    /// Converts the request parameters into HTTP query parameters.
    ///
    /// # Returns
    /// A vector of query parameter tuples containing both pagination and filter criteria.
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = self.pagination.to_query_params();
        params.extend(self.filters.to_query_params());
        params
    }
}

impl Default for ListAppliedCouponsRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Input for applying a coupon to a customer.
///
/// This struct contains the data needed to apply a coupon to a customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyCouponInput {
    pub external_customer_id: String,
    pub coupon_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<AppliedCouponFrequency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_rate: Option<String>,
}

impl ApplyCouponInput {
    /// Creates a new apply coupon input with the required fields.
    ///
    /// # Arguments
    /// * `external_customer_id` - The external ID of the customer
    /// * `coupon_code` - The code of the coupon to apply
    ///
    /// # Returns
    /// A new `ApplyCouponInput` instance
    pub fn new(external_customer_id: String, coupon_code: String) -> Self {
        Self {
            external_customer_id,
            coupon_code,
            frequency: None,
            frequency_duration: None,
            amount_cents: None,
            amount_currency: None,
            percentage_rate: None,
        }
    }

    /// Sets the frequency for the coupon application.
    ///
    /// # Arguments
    /// * `frequency` - The frequency type (once, recurring, or forever)
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_frequency(mut self, frequency: AppliedCouponFrequency) -> Self {
        self.frequency = Some(frequency);
        self
    }

    /// Sets the frequency duration for recurring coupons.
    ///
    /// # Arguments
    /// * `duration` - The number of billing periods the coupon applies to
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_frequency_duration(mut self, duration: i32) -> Self {
        self.frequency_duration = Some(duration);
        self
    }

    /// Sets a fixed amount discount.
    ///
    /// # Arguments
    /// * `amount_cents` - The discount amount in cents
    /// * `currency` - The currency code (e.g., "USD")
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_fixed_amount(mut self, amount_cents: i64, currency: String) -> Self {
        self.amount_cents = Some(amount_cents);
        self.amount_currency = Some(currency);
        self
    }

    /// Sets a percentage discount.
    ///
    /// # Arguments
    /// * `percentage_rate` - The discount percentage (e.g., "10.5" for 10.5%)
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_percentage_rate(mut self, percentage_rate: String) -> Self {
        self.percentage_rate = Some(percentage_rate);
        self
    }
}

/// Request for applying a coupon to a customer.
///
/// This struct wraps the apply coupon input in the expected API format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyCouponRequest {
    pub applied_coupon: ApplyCouponInput,
}

impl ApplyCouponRequest {
    /// Creates a new apply coupon request.
    ///
    /// # Arguments
    /// * `input` - The apply coupon input data
    ///
    /// # Returns
    /// A new `ApplyCouponRequest` instance
    pub fn new(input: ApplyCouponInput) -> Self {
        Self {
            applied_coupon: input,
        }
    }
}
