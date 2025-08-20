use lago_types::{
    error::{LagoError, Result},
    requests::customer::{CreateCustomerRequest, GetCustomerRequest, ListCustomersRequest},
    responses::customer::{CreateCustomerResponse, GetCustomerResponse, ListCustomersResponse},
};
use url::Url;

use crate::client::LagoClient;

impl LagoClient {
    /// Retrieves a list of customers with optional filtering parameters
    ///
    /// # Arguments
    /// * `request` - Optional filtering parameters for the customer list
    ///
    /// # Returns
    /// A `Result` containing the list of customers or an error
    pub async fn list_customers(
        &self,
        request: Option<ListCustomersRequest>,
    ) -> Result<ListCustomersResponse> {
        let request = request.unwrap_or_default();
        let region = self.config.region()?;
        let mut url = Url::parse(&format!("{}/customers", region.endpoint()))
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

    /// Retrieves a specific customer by their external ID
    ///
    /// # Arguments
    /// * `request` - The request containing the customer external ID to retrieve
    ///
    /// # Returns
    /// A `Result` containing the customer data or an error
    pub async fn get_customer(&self, request: GetCustomerRequest) -> Result<GetCustomerResponse> {
        let region = self.config.region()?;
        let url = format!("{}/customers/{}", region.endpoint(), request.external_id);

        self.make_request("GET", &url, None::<&()>).await
    }

    /// Creates a new customer
    ///
    /// # Arguments
    /// * `request` - The request containing the customer data to create
    ///
    /// # Returns
    /// A `Result` containing the created customer data or an error
    pub async fn create_customer(
        &self,
        request: CreateCustomerRequest,
    ) -> Result<CreateCustomerResponse> {
        let region = self.config.region()?;
        let url = format!("{}/customers", region.endpoint());

        self.make_request("POST", &url, Some(&request)).await
    }
}
