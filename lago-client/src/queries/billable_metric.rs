use lago_types::{
    error::{LagoError, Result},
    requests::billable_metric::{
        CreateBillableMetricRequest, GetBillableMetricRequest, ListBillableMetricsRequest,
    },
    responses::billable_metric::{
        CreateBillableMetricResponse, GetBillableMetricResponse, ListBillableMetricsResponse,
    },
};
use url::Url;

use crate::client::LagoClient;

impl LagoClient {
    /// Retrieves a list of billable metrics with optional filtering parameters
    ///
    /// # Arguments
    /// * `request` - Optional filtering parameters for the billable metric list
    ///
    /// # Returns
    /// A `Result` containing the list of billable metrics or an error
    pub async fn list_billable_metrics(
        &self,
        request: Option<ListBillableMetricsRequest>,
    ) -> Result<ListBillableMetricsResponse> {
        let request = request.unwrap_or_default();
        let region = self.config.region()?;
        let mut url = Url::parse(&format!("{}/billable_metrics", region.endpoint()))
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

    /// Retrieves a specific billable metric by its code
    ///
    /// # Arguments
    /// * `request` - The request containing the billable metric code to retrieve
    ///
    /// # Returns
    /// A `Result` containing the billable metric data or an error
    pub async fn get_billable_metric(
        &self,
        request: GetBillableMetricRequest,
    ) -> Result<GetBillableMetricResponse> {
        let region = self.config.region()?;
        let url = format!("{}/billable_metrics/{}", region.endpoint(), request.code);

        self.make_request("GET", &url, None::<&()>).await
    }

    /// Creates a new billable metric
    ///
    /// # Arguments
    /// * `request` - The request containing the billable metric data to create
    ///
    /// # Returns
    /// A `Result` containing the created billable metric data or an error
    pub async fn create_billable_metric(
        &self,
        request: CreateBillableMetricRequest,
    ) -> Result<CreateBillableMetricResponse> {
        let region = self.config.region()?;
        let url = format!("{}/billable_metrics", region.endpoint());

        self.make_request("POST", &url, Some(&request)).await
    }
}
