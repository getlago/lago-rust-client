/*
 * Lago API documentation
 *
 * Lago API allows your application to push customer information and metrics (events) from your application to the billing application.
 *
 * The version of the OpenAPI document: 0.21.0-beta
 * Contact: tech@getlago.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CouponInputCoupon {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Coupon type
    #[serde(rename = "coupon_type", skip_serializing_if = "Option::is_none")]
    pub coupon_type: Option<CouponType>,
    #[serde(rename = "amount_cents", skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i32>,
    #[serde(rename = "amount_currency", skip_serializing_if = "Option::is_none")]
    pub amount_currency: Option<String>,
    #[serde(rename = "reusable", skip_serializing_if = "Option::is_none")]
    pub reusable: Option<bool>,
    #[serde(rename = "percentage_rate", skip_serializing_if = "Option::is_none")]
    pub percentage_rate: Option<f32>,
    /// Frequency type
    #[serde(rename = "frequency", skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    #[serde(rename = "frequency_duration", skip_serializing_if = "Option::is_none")]
    pub frequency_duration: Option<i32>,
    #[serde(rename = "expiration_at", skip_serializing_if = "Option::is_none")]
    pub expiration_at: Option<String>,
    /// Expiration type
    #[serde(rename = "expiration", skip_serializing_if = "Option::is_none")]
    pub expiration: Option<Expiration>,
}

impl CouponInputCoupon {
    pub fn new() -> CouponInputCoupon {
        CouponInputCoupon {
            name: None,
            code: None,
            coupon_type: None,
            amount_cents: None,
            amount_currency: None,
            reusable: None,
            percentage_rate: None,
            frequency: None,
            frequency_duration: None,
            expiration_at: None,
            expiration: None,
        }
    }
}

/// Coupon type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CouponType {
    #[serde(rename = "fixed_amount")]
    FixedAmount,
    #[serde(rename = "percentage")]
    Percentage,
}

impl Default for CouponType {
    fn default() -> CouponType {
        Self::FixedAmount
    }
}
/// Frequency type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Frequency {
    #[serde(rename = "once")]
    Once,
    #[serde(rename = "recurring")]
    Recurring,
}

impl Default for Frequency {
    fn default() -> Frequency {
        Self::Once
    }
}
/// Expiration type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Expiration {
    #[serde(rename = "no_expiration")]
    NoExpiration,
    #[serde(rename = "time_limit")]
    TimeLimit,
}

impl Default for Expiration {
    fn default() -> Expiration {
        Self::NoExpiration
    }
}

