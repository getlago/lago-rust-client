use serde::{Deserialize, Serialize};

use crate::models::{CouponExpiration, CouponFrequency, CouponType};

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
