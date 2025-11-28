use serde::{Deserialize, Serialize};

use crate::filters::{common::ListFilters, coupon::CouponFilter};
use crate::models::{CouponExpiration, CouponFrequency, CouponType, PaginationParams};

/// Request parameters for retrieving a specific coupon.
#[derive(Debug, Clone)]
pub struct GetCouponRequest {
    pub code: String,
}

impl GetCouponRequest {
    /// Creates a new get coupon request.
    ///
    /// # Arguments
    /// * `code` - The unique code of the coupon to retrieve
    ///
    /// # Returns
    /// A new `GetCouponRequest` instance
    pub fn new(code: String) -> Self {
        Self { code }
    }
}

/// Input for creating a coupon.
///
/// This struct contains the data needed to create a new coupon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCouponInput {
    pub name: String,
    pub code: String,
    pub coupon_type: CouponType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_rate: Option<String>,
    pub frequency: CouponFrequency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reusable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_plans: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_codes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_billable_metrics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable_metric_codes: Option<Vec<String>>,
    pub expiration: CouponExpiration,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_at: Option<String>,
}

impl CreateCouponInput {
    /// Creates a new fixed amount coupon input.
    ///
    /// # Arguments
    /// * `name` - The name of the coupon
    /// * `code` - The unique code for the coupon
    /// * `amount_cents` - The discount amount in cents
    /// * `amount_currency` - The currency code (e.g., "USD")
    /// * `frequency` - The frequency of the coupon application
    /// * `expiration` - The expiration policy for the coupon
    ///
    /// # Returns
    /// A new `CreateCouponInput` instance for a fixed amount coupon
    pub fn fixed_amount(
        name: String,
        code: String,
        amount_cents: i64,
        amount_currency: String,
        frequency: CouponFrequency,
        expiration: CouponExpiration,
    ) -> Self {
        Self {
            name,
            code,
            coupon_type: CouponType::FixedAmount,
            amount_cents: Some(amount_cents),
            amount_currency: Some(amount_currency),
            percentage_rate: None,
            frequency,
            frequency_duration: None,
            reusable: None,
            limited_plans: None,
            plan_codes: None,
            limited_billable_metrics: None,
            billable_metric_codes: None,
            expiration,
            expiration_at: None,
        }
    }

    /// Creates a new percentage coupon input.
    ///
    /// # Arguments
    /// * `name` - The name of the coupon
    /// * `code` - The unique code for the coupon
    /// * `percentage_rate` - The discount percentage (e.g., "10.5" for 10.5%)
    /// * `frequency` - The frequency of the coupon application
    /// * `expiration` - The expiration policy for the coupon
    ///
    /// # Returns
    /// A new `CreateCouponInput` instance for a percentage coupon
    pub fn percentage(
        name: String,
        code: String,
        percentage_rate: String,
        frequency: CouponFrequency,
        expiration: CouponExpiration,
    ) -> Self {
        Self {
            name,
            code,
            coupon_type: CouponType::Percentage,
            amount_cents: None,
            amount_currency: None,
            percentage_rate: Some(percentage_rate),
            frequency,
            frequency_duration: None,
            reusable: None,
            limited_plans: None,
            plan_codes: None,
            limited_billable_metrics: None,
            billable_metric_codes: None,
            expiration,
            expiration_at: None,
        }
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

    /// Sets whether the coupon is reusable.
    ///
    /// # Arguments
    /// * `reusable` - Whether the coupon can be applied multiple times
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_reusable(mut self, reusable: bool) -> Self {
        self.reusable = Some(reusable);
        self
    }

    /// Limits the coupon to specific plans.
    ///
    /// # Arguments
    /// * `plan_codes` - The plan codes this coupon applies to
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_limited_plans(mut self, plan_codes: Vec<String>) -> Self {
        self.limited_plans = Some(true);
        self.plan_codes = Some(plan_codes);
        self
    }

    /// Limits the coupon to specific billable metrics.
    ///
    /// # Arguments
    /// * `billable_metric_codes` - The billable metric codes this coupon applies to
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_limited_billable_metrics(mut self, billable_metric_codes: Vec<String>) -> Self {
        self.limited_billable_metrics = Some(true);
        self.billable_metric_codes = Some(billable_metric_codes);
        self
    }

    /// Sets the expiration timestamp for the coupon (for TimeLimit expiration).
    ///
    /// # Arguments
    /// * `expiration_at` - The expiration timestamp (ISO 8601 format)
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_expiration_at(mut self, expiration_at: String) -> Self {
        self.expiration_at = Some(expiration_at);
        self
    }
}

/// Request for creating a coupon.
///
/// This struct wraps the create coupon input in the expected API format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCouponRequest {
    pub coupon: CreateCouponInput,
}

impl CreateCouponRequest {
    /// Creates a new create coupon request.
    ///
    /// # Arguments
    /// * `input` - The create coupon input data
    ///
    /// # Returns
    /// A new `CreateCouponRequest` instance
    pub fn new(input: CreateCouponInput) -> Self {
        Self { coupon: input }
    }
}

/// Request parameters for listing coupons.
///
/// This struct combines pagination parameters and coupon-specific filters
/// to build a comprehensive request for retrieving coupon lists.
#[derive(Debug, Clone)]
pub struct ListCouponsRequest {
    pub pagination: PaginationParams,
    pub filters: CouponFilter,
}

impl ListCouponsRequest {
    /// Creates a new empty list coupons request.
    ///
    /// # Returns
    /// A new `ListCouponsRequest` instance with default pagination and no filters.
    pub fn new() -> Self {
        Self {
            pagination: PaginationParams::default(),
            filters: CouponFilter::default(),
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

    /// Sets the coupon filters for the request.
    ///
    /// # Arguments
    /// * `filters` - The coupon filters to apply
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_filters(mut self, filters: CouponFilter) -> Self {
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

impl Default for ListCouponsRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Request parameters for deleting a coupon.
#[derive(Debug, Clone)]
pub struct DeleteCouponRequest {
    pub code: String,
}

impl DeleteCouponRequest {
    /// Creates a new delete coupon request.
    ///
    /// # Arguments
    /// * `code` - The unique code of the coupon to delete
    ///
    /// # Returns
    /// A new `DeleteCouponRequest` instance
    pub fn new(code: String) -> Self {
        Self { code }
    }
}

/// Input for updating a coupon.
///
/// This struct contains the data that can be updated on an existing coupon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCouponInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_type: Option<CouponType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<CouponFrequency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reusable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_plans: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_codes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_billable_metrics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable_metric_codes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<CouponExpiration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_at: Option<String>,
}

impl UpdateCouponInput {
    /// Creates a new empty update coupon input.
    ///
    /// # Returns
    /// A new `UpdateCouponInput` instance with all fields set to None.
    pub fn new() -> Self {
        Self {
            name: None,
            coupon_type: None,
            amount_cents: None,
            amount_currency: None,
            percentage_rate: None,
            frequency: None,
            frequency_duration: None,
            reusable: None,
            limited_plans: None,
            plan_codes: None,
            limited_billable_metrics: None,
            billable_metric_codes: None,
            expiration: None,
            expiration_at: None,
        }
    }

    /// Sets the name for the coupon.
    ///
    /// # Arguments
    /// * `name` - The new name for the coupon
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the coupon type to fixed amount with the given values.
    ///
    /// # Arguments
    /// * `amount_cents` - The discount amount in cents
    /// * `amount_currency` - The currency code (e.g., "USD")
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_fixed_amount(mut self, amount_cents: i64, amount_currency: String) -> Self {
        self.coupon_type = Some(CouponType::FixedAmount);
        self.amount_cents = Some(amount_cents);
        self.amount_currency = Some(amount_currency);
        self.percentage_rate = None;
        self
    }

    /// Sets the coupon type to percentage with the given rate.
    ///
    /// # Arguments
    /// * `percentage_rate` - The discount percentage (e.g., "10.5" for 10.5%)
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_percentage_rate(mut self, percentage_rate: String) -> Self {
        self.coupon_type = Some(CouponType::Percentage);
        self.percentage_rate = Some(percentage_rate);
        self.amount_cents = None;
        self.amount_currency = None;
        self
    }

    /// Sets the frequency for the coupon.
    ///
    /// # Arguments
    /// * `frequency` - The frequency of the coupon application
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_frequency(mut self, frequency: CouponFrequency) -> Self {
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

    /// Sets whether the coupon is reusable.
    ///
    /// # Arguments
    /// * `reusable` - Whether the coupon can be applied multiple times
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_reusable(mut self, reusable: bool) -> Self {
        self.reusable = Some(reusable);
        self
    }

    /// Limits the coupon to specific plans.
    ///
    /// # Arguments
    /// * `plan_codes` - The plan codes this coupon applies to
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_limited_plans(mut self, plan_codes: Vec<String>) -> Self {
        self.limited_plans = Some(true);
        self.plan_codes = Some(plan_codes);
        self
    }

    /// Limits the coupon to specific billable metrics.
    ///
    /// # Arguments
    /// * `billable_metric_codes` - The billable metric codes this coupon applies to
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_limited_billable_metrics(mut self, billable_metric_codes: Vec<String>) -> Self {
        self.limited_billable_metrics = Some(true);
        self.billable_metric_codes = Some(billable_metric_codes);
        self
    }

    /// Sets the expiration policy for the coupon.
    ///
    /// # Arguments
    /// * `expiration` - The expiration policy
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_expiration(mut self, expiration: CouponExpiration) -> Self {
        self.expiration = Some(expiration);
        self
    }

    /// Sets the expiration timestamp for the coupon (for TimeLimit expiration).
    ///
    /// # Arguments
    /// * `expiration_at` - The expiration timestamp (ISO 8601 format)
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_expiration_at(mut self, expiration_at: String) -> Self {
        self.expiration_at = Some(expiration_at);
        self
    }
}

impl Default for UpdateCouponInput {
    fn default() -> Self {
        Self::new()
    }
}

/// Request for updating a coupon.
///
/// This struct wraps the update coupon input in the expected API format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCouponRequest {
    pub code: String,
    pub coupon: UpdateCouponInput,
}

impl UpdateCouponRequest {
    /// Creates a new update coupon request.
    ///
    /// # Arguments
    /// * `code` - The unique code of the coupon to update
    /// * `input` - The update coupon input data
    ///
    /// # Returns
    /// A new `UpdateCouponRequest` instance
    pub fn new(code: String, input: UpdateCouponInput) -> Self {
        Self {
            code,
            coupon: input,
        }
    }
}
