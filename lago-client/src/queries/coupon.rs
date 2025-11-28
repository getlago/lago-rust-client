use lago_types::{
    error::{LagoError, Result},
    requests::coupon::{
        CreateCouponRequest, DeleteCouponRequest, GetCouponRequest, ListCouponsRequest,
        UpdateCouponRequest,
    },
    responses::coupon::{
        CreateCouponResponse, DeleteCouponResponse, GetCouponResponse, ListCouponsResponse,
        UpdateCouponResponse,
    },
};
use serde::Serialize;
use url::Url;

use crate::client::LagoClient;

/// Internal struct for serializing update request body (without the code field)
#[derive(Serialize)]
struct UpdateCouponBody {
    coupon: lago_types::requests::coupon::UpdateCouponInput,
}

impl LagoClient {
    /// Retrieves a list of coupons with optional filtering parameters
    ///
    /// # Arguments
    /// * `request` - Optional filtering parameters for the coupon list
    ///
    /// # Returns
    /// A `Result` containing the list of coupons or an error
    pub async fn list_coupons(
        &self,
        request: Option<ListCouponsRequest>,
    ) -> Result<ListCouponsResponse> {
        let request = request.unwrap_or_default();
        let region = self.config.region()?;
        let mut url = Url::parse(&format!("{}/coupons", region.endpoint()))
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

    /// Updates an existing coupon
    ///
    /// # Arguments
    /// * `request` - The request containing the coupon code and update data
    ///
    /// # Returns
    /// A `Result` containing the updated coupon data or an error
    pub async fn update_coupon(
        &self,
        request: UpdateCouponRequest,
    ) -> Result<UpdateCouponResponse> {
        let region = self.config.region()?;
        let url = format!("{}/coupons/{}", region.endpoint(), request.code);

        let body = UpdateCouponBody {
            coupon: request.coupon,
        };

        self.make_request("PUT", &url, Some(&body)).await
    }

    /// Deletes a coupon by its code
    ///
    /// # Arguments
    /// * `request` - The request containing the coupon code to delete
    ///
    /// # Returns
    /// A `Result` containing the deleted coupon data or an error
    pub async fn delete_coupon(
        &self,
        request: DeleteCouponRequest,
    ) -> Result<DeleteCouponResponse> {
        let region = self.config.region()?;
        let url = format!("{}/coupons/{}", region.endpoint(), request.code);

        self.make_request("DELETE", &url, None::<&()>).await
    }
}
