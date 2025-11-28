use serde::{Deserialize, Serialize};

use crate::models::{AppliedCoupon, PaginationMeta};

/// Response for listing applied coupons.
///
/// This struct represents the response returned when requesting a list of
/// applied coupons, including pagination metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAppliedCouponsResponse {
    pub applied_coupons: Vec<AppliedCoupon>,
    pub meta: PaginationMeta,
}

/// Response for applying a coupon to a customer.
///
/// This struct represents the response returned when successfully applying
/// a coupon to a customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyCouponResponse {
    pub applied_coupon: AppliedCoupon,
}
