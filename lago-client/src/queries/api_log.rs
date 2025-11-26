use lago_types::{
    error::{LagoError, Result},
    requests::api_log::{GetApiLogRequest, ListApiLogsRequest},
    responses::api_log::{GetApiLogResponse, ListApiLogsResponse},
};
use url::Url;

use crate::client::LagoClient;

/// API log-related operations for the Lago client
impl LagoClient {
    /// Retrieves a list of API logs with optional filtering parameters
    ///
    /// # Arguments
    /// * `request` - Optional filtering parameters for the API log list
    ///
    /// # Returns
    /// A `Result` containing the list of API logs or an error
    pub async fn list_api_logs(
        &self,
        request: Option<ListApiLogsRequest>,
    ) -> Result<ListApiLogsResponse> {
        let request = request.unwrap_or_default();
        let region = self.config.region()?;
        let mut url = Url::parse(&format!("{}/api_logs", region.endpoint()))
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

    /// Retrieves a specific API log by its request ID
    ///
    /// # Arguments
    /// * `request` - The request containing the API log request ID to retrieve
    ///
    /// # Returns
    /// A `Result` containing the API log data or an error
    pub async fn get_api_log(&self, request: GetApiLogRequest) -> Result<GetApiLogResponse> {
        let region = self.config.region()?;
        let url = format!("{}/api_logs/{}", region.endpoint(), request.request_id);
        self.make_request("GET", &url, None::<&()>).await
    }
}
