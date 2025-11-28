use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use uuid::Uuid;

/// Represents an applied coupon in the Lago billing system.
///
/// This struct contains all information about a coupon that has been applied
/// to a customer, including amounts, status, and frequency settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedCoupon {
    pub lago_id: Uuid,
    pub lago_coupon_id: Uuid,
    pub coupon_code: String,
    pub coupon_name: String,
    pub lago_customer_id: Uuid,
    pub external_customer_id: String,
    pub status: AppliedCouponStatus,
    pub frequency: AppliedCouponFrequency,
    pub created_at: DateTime<Utc>,
    pub amount_cents: Option<i64>,
    pub amount_cents_remaining: Option<i64>,
    pub amount_currency: Option<String>,
    pub percentage_rate: Option<String>,
    pub frequency_duration: Option<i32>,
    pub frequency_duration_remaining: Option<i32>,
    pub expiration_at: Option<DateTime<Utc>>,
    pub terminated_at: Option<DateTime<Utc>>,
}

/// Defines the status of an applied coupon.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AppliedCouponStatus {
    Active,
    Terminated,
}

/// Defines the frequency type for a coupon application.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AppliedCouponFrequency {
    Once,
    Recurring,
    Forever,
}
