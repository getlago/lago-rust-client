use serde::{Deserialize, Serialize};

use crate::models::Coupon;

/// Response for retrieving a coupon.
///
/// This struct represents the response returned when retrieving a coupon
/// by its code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCouponResponse {
    pub coupon: Coupon,
}

/// Response for creating a coupon.
///
/// This struct represents the response returned when successfully creating
/// a new coupon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCouponResponse {
    pub coupon: Coupon,
}
