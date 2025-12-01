use lago_types::{
    error::{LagoError, Result},
    requests::customer_usage::GetCustomerCurrentUsageRequest,
    responses::customer_usage::GetCustomerCurrentUsageResponse,
};
use url::Url;

use crate::client::LagoClient;

impl LagoClient {
    /// Retrieves the current usage for a customer's subscription
    ///
    /// This endpoint enables the retrieval of usage-based billing data for a customer
    /// within the current billing period.
    ///
    /// # Arguments
    /// * `request` - The request containing the customer and subscription IDs
    ///
    /// # Returns
    /// A `Result` containing the customer usage data or an error
    pub async fn get_customer_current_usage(
        &self,
        request: GetCustomerCurrentUsageRequest,
    ) -> Result<GetCustomerCurrentUsageResponse> {
        let region = self.config.region()?;
        let mut url = Url::parse(&format!(
            "{}/customers/{}/current_usage",
            region.endpoint(),
            request.external_customer_id
        ))
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
}
