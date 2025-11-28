use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use uuid::Uuid;

/// Represents a coupon in the Lago billing system.
///
/// This struct contains all information about a coupon that can be
/// applied to customers for discounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coupon {
    pub lago_id: Uuid,
    pub name: String,
    pub code: String,
    pub coupon_type: CouponType,
    pub amount_cents: Option<i64>,
    pub amount_currency: Option<String>,
    pub percentage_rate: Option<String>,
    pub frequency: CouponFrequency,
    pub frequency_duration: Option<i32>,
    pub reusable: bool,
    pub limited_plans: bool,
    pub plan_codes: Option<Vec<String>>,
    pub limited_billable_metrics: bool,
    pub billable_metric_codes: Option<Vec<String>>,
    pub expiration: CouponExpiration,
    pub expiration_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub terminated_at: Option<DateTime<Utc>>,
}

/// Defines the type of coupon discount.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CouponType {
    FixedAmount,
    Percentage,
}

/// Defines the frequency type for a coupon.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CouponFrequency {
    Once,
    Recurring,
    Forever,
}

/// Defines the expiration policy for a coupon.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CouponExpiration {
    NoExpiration,
    TimeLimit,
}
