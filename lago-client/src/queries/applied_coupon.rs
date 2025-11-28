use lago_types::{
    error::{LagoError, Result},
    requests::applied_coupon::{ApplyCouponRequest, ListAppliedCouponsRequest},
    responses::applied_coupon::{ApplyCouponResponse, ListAppliedCouponsResponse},
};
use url::Url;

use crate::client::LagoClient;

impl LagoClient {
    /// Retrieves a list of applied coupons with optional filtering parameters
    ///
    /// # Arguments
    /// * `request` - Optional filtering parameters for the applied coupon list
    ///
    /// # Returns
    /// A `Result` containing the list of applied coupons or an error
    pub async fn list_applied_coupons(
        &self,
        request: Option<ListAppliedCouponsRequest>,
    ) -> Result<ListAppliedCouponsResponse> {
        let request = request.unwrap_or_default();
        let region = self.config.region()?;
        let mut url = Url::parse(&format!("{}/applied_coupons", region.endpoint()))
            .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        let query_params = request.to_query_params();

        if !query_params.is_empty() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("&");
            url.set_query(Some(&query_string));
        }

        self.make_request("GET", url.as_str(), None::<&()>).await
    }

    /// Applies a coupon to a customer
    ///
    /// # Arguments
    /// * `request` - The request containing the coupon application data
    ///
    /// # Returns
    /// A `Result` containing the applied coupon data or an error
    pub async fn apply_coupon(&self, request: ApplyCouponRequest) -> Result<ApplyCouponResponse> {
        let region = self.config.region()?;
        let url = format!("{}/applied_coupons", region.endpoint());

        self.make_request("POST", &url, Some(&request)).await
    }
}
