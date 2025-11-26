use lago_types::{
    error::{LagoError, Result},
    requests::activity_log::{GetActivityLogRequest, ListActivityLogsRequest},
    responses::activity_log::{GetActivityLogResponse, ListActivityLogsResponse},
};
use url::Url;

use crate::client::LagoClient;

/// Activity log-related operations for the Lago client
impl LagoClient {
    /// Retrieves a list of activity logs with optional filtering parameters
    ///
    /// # Arguments
    /// * `request` - Optional filtering parameters for the activity log list
    ///
    /// # Returns
    /// A `Result` containing the list of activity logs or an error
    pub async fn list_activity_logs(
        &self,
        request: Option<ListActivityLogsRequest>,
    ) -> Result<ListActivityLogsResponse> {
        let request = request.unwrap_or_default();
        let region = self.config.region()?;
        let mut url = Url::parse(&format!("{}/activity_logs", region.endpoint()))
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

    /// Retrieves a specific activity log by its activity ID
    ///
    /// # Arguments
    /// * `request` - The request containing the activity log ID to retrieve
    ///
    /// # Returns
    /// A `Result` containing the activity log data or an error
    pub async fn get_activity_log(
        &self,
        request: GetActivityLogRequest,
    ) -> Result<GetActivityLogResponse> {
        let region = self.config.region()?;
        let url = format!("{}/activity_logs/{}", region.endpoint(), request.activity_id);
        self.make_request("GET", &url, None::<&()>).await
    }
}
