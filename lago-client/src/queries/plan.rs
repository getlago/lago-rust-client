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
    /// Lists all plans with optional filters and pagination.
    ///
    /// # Arguments
    /// * `request` - Optional request with pagination and filter parameters
    ///
    /// # Returns
    /// A `Result` containing the list of plans or an error
    pub async fn list_plans(&self, request: Option<ListPlansRequest>) -> Result<ListPlansResponse> {
        let region = self.config.region()?;
        let mut url = Url::parse(&format!("{}/plans", region.endpoint()))
            .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        if let Some(req) = request {
            let query_params = req.to_query_params();

            if !query_params.is_empty() {
                let query_string = query_params
                    .iter()
                    .map(|(k, v)| format!("{k}={v}"))
                    .collect::<Vec<_>>()
                    .join("&");
                url.set_query(Some(&query_string));
            }
        }

        self.make_request("GET", url.as_str(), None::<&()>).await
    }

    /// Retrieves a specific plan by its code.
    ///
    /// # Arguments
    /// * `request` - The request containing the plan's code
    ///
    /// # Returns
    /// A `Result` containing the plan or an error
    pub async fn get_plan(&self, request: GetPlanRequest) -> Result<GetPlanResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!("{}/plans/{}", region.endpoint(), request.code))
            .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("GET", url.as_str(), None::<&()>).await
    }

    /// Creates a new plan.
    ///
    /// # Arguments
    /// * `request` - The plan creation request with all plan details
    ///
    /// # Returns
    /// A `Result` containing the created plan or an error
    pub async fn create_plan(&self, request: CreatePlanRequest) -> Result<CreatePlanResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!("{}/plans", region.endpoint()))
            .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("POST", url.as_str(), Some(&request))
            .await
    }

    /// Updates an existing plan.
    ///
    /// # Arguments
    /// * `request` - The plan update request with the code and update data
    ///
    /// # Returns
    /// A `Result` containing the updated plan or an error
    pub async fn update_plan(&self, request: UpdatePlanRequest) -> Result<UpdatePlanResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!("{}/plans/{}", region.endpoint(), request.code))
            .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("PUT", url.as_str(), Some(&request)).await
    }

    /// Deletes a plan.
    ///
    /// # Arguments
    /// * `request` - The delete request containing the plan's code
    ///
    /// # Returns
    /// A `Result` containing the deleted plan or an error
    pub async fn delete_plan(&self, request: DeletePlanRequest) -> Result<DeletePlanResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!("{}/plans/{}", region.endpoint(), request.code))
            .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("DELETE", url.as_str(), None::<&()>).await
    }
}
