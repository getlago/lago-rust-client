use lago_types::{
    error::Result,
    requests::coupon::{CreateCouponRequest, GetCouponRequest},
    responses::coupon::{CreateCouponResponse, GetCouponResponse},
};

use crate::client::LagoClient;

impl LagoClient {
    /// Retrieves a coupon by its code
    ///
    /// # Arguments
    /// * `request` - The request containing the coupon code to retrieve
    ///
    /// # Returns
    /// A `Result` containing the coupon data or an error
    pub async fn get_coupon(&self, request: GetCouponRequest) -> Result<GetCouponResponse> {
        let region = self.config.region()?;
        let url = format!("{}/coupons/{}", region.endpoint(), request.code);

        self.make_request("GET", &url, None::<&()>).await
    }

    /// Creates a new coupon
    ///
    /// # Arguments
    /// * `request` - The request containing the coupon data to create
    ///
    /// # Returns
    /// A `Result` containing the created coupon data or an error
    pub async fn create_coupon(
        &self,
        request: CreateCouponRequest,
    ) -> Result<CreateCouponResponse> {
        let region = self.config.region()?;
        let url = format!("{}/coupons", region.endpoint());

        self.make_request("POST", &url, Some(&request)).await
    }
}
