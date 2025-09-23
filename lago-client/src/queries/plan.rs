use lago_types::{
    error::{LagoError, Result},
    requests::plan::{
        CreatePlanRequest, DeletePlanRequest, GetPlanRequest, ListPlansRequest, UpdatePlanRequest,
    },
    responses::plan::{
        CreatePlanResponse, DeletePlanResponse, GetPlanResponse, ListPlansResponse,
        UpdatePlanResponse,
    },
};
use url::Url;

use crate::client::LagoClient;

impl LagoClient {
    /// Retrieves a list of plans with optional filtering parameters
    ///
    /// # Arguments
    /// * `request` - Optional filtering parameters for the plan list
    ///
    /// # Returns
    /// A `Result` containing the list of plans or an error
    pub async fn list_plans(&self, request: Option<ListPlansRequest>) -> Result<ListPlansResponse> {
        let request = request.unwrap_or_default();
        let region = self.config.region()?;
        let mut url = Url::parse(&format!("{}/plans", region.endpoint()))
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

    /// Retrieves a specific plan by its code
    ///
    /// # Arguments
    /// * `request` - The request containing the plan code to retrieve
    ///
    /// # Returns
    /// A `Result` containing the plan data or an error
    pub async fn get_plan(&self, request: GetPlanRequest) -> Result<GetPlanResponse> {
        let region = self.config.region()?;
        let url = format!("{}/plans/{}", region.endpoint(), request.code);

        self.make_request("GET", &url, None::<&()>).await
    }

    /// Creates a new plan
    ///
    /// # Arguments
    /// * `request` - The request containing the plan data to create
    ///
    /// # Returns
    /// A `Result` containing the created plan data or an error
    pub async fn create_plan(&self, request: CreatePlanRequest) -> Result<CreatePlanResponse> {
        let region = self.config.region()?;
        let url = format!("{}/plans", region.endpoint());

        self.make_request("POST", &url, Some(&request)).await
    }

    /// Updates an existing plan
    ///
    /// # Arguments
    /// * `code` - The code of the plan to update
    /// * `request` - The request containing the plan data to update
    ///
    /// # Returns
    /// A `Result` containing the updated plan data or an error
    pub async fn update_plan(
        &self,
        code: &str,
        request: UpdatePlanRequest,
    ) -> Result<UpdatePlanResponse> {
        let region = self.config.region()?;
        let url = format!("{}/plans/{}", region.endpoint(), code);

        self.make_request("PUT", &url, Some(&request)).await
    }

    /// Deletes a plan
    ///
    /// # Arguments
    /// * `request` - The request containing the plan code to delete
    ///
    /// # Returns
    /// A `Result` containing the deleted plan data or an error
    pub async fn delete_plan(&self, request: DeletePlanRequest) -> Result<DeletePlanResponse> {
        let region = self.config.region()?;
        let url = format!("{}/plans/{}", region.endpoint(), request.code);

        self.make_request("DELETE", &url, None::<&()>).await
    }
}
