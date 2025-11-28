use serde::{Deserialize, Serialize};

use crate::models::{Coupon, PaginationMeta};

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

/// Response for listing coupons.
///
/// This struct represents the response returned when requesting a list of
/// coupons, including pagination metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCouponsResponse {
    pub coupons: Vec<Coupon>,
    pub meta: PaginationMeta,
}

/// Response for updating a coupon.
///
/// This struct represents the response returned when successfully updating
/// an existing coupon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCouponResponse {
    pub coupon: Coupon,
}

/// Response for deleting a coupon.
///
/// This struct represents the response returned when successfully deleting
/// a coupon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCouponResponse {
    pub coupon: Coupon,
}
