use lago_types::{
    error::{LagoError, Result},
    requests::fee::{GetFeeRequest, ListFeesRequest},
    responses::fee::{GetFeeResponse, ListFeesResponse},
};
use url::Url;

use crate::client::LagoClient;

/// Fee-related operations for the Lago client.
impl LagoClient {
    /// Retrieves a list of fees with optional filtering parameters.
    ///
    /// Useful for revenue and MRR reporting that needs fee-level granularity —
    /// invoices alone don't reveal which fees come from subscriptions, usage
    /// charges (e.g., seats, storage), add-ons, credits, or commitments.
    ///
    /// # Arguments
    /// * `request` - Optional filtering parameters for the fee list. When `None`,
    ///   uses default pagination and no filters.
    ///
    /// # Returns
    /// A `Result` containing the list of fees and pagination metadata, or an error.
    pub async fn list_fees(&self, request: Option<ListFeesRequest>) -> Result<ListFeesResponse> {
        let request = request.unwrap_or_default();
        let region = self.config.region()?;
        let mut url = Url::parse(&format!("{}/fees", region.endpoint()))
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

    /// Retrieves a specific fee by its Lago ID (UUID).
    ///
    /// # Arguments
    /// * `request` - The request containing the fee's Lago ID
    ///
    /// # Returns
    /// A `Result` containing the fee data or an error.
    pub async fn get_fee(&self, request: GetFeeRequest) -> Result<GetFeeResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!("{}/fees/{}", region.endpoint(), request.fee_id))
            .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("GET", url.as_str(), None::<&()>).await
    }
}
