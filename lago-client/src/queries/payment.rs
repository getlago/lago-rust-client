use lago_types::{
    error::{LagoError, Result},
    requests::payment::{
        CreatePaymentRequest, GetPaymentRequest, ListCustomerPaymentsRequest, ListPaymentsRequest,
    },
    responses::payment::{CreatePaymentResponse, GetPaymentResponse, ListPaymentsResponse},
};
use url::Url;

use crate::client::LagoClient;

/// Payment-related operations for the Lago client
impl LagoClient {
    /// Retrieves a list of payments with optional filtering parameters
    ///
    /// # Arguments
    /// * `request` - Optional filtering parameters for the payment list
    ///
    /// # Returns
    /// A `Result` containing the list of payments or an error
    pub async fn list_payments(
        &self,
        request: Option<ListPaymentsRequest>,
    ) -> Result<ListPaymentsResponse> {
        let request = request.unwrap_or_default();
        let region = self.config.region()?;
        let mut url = Url::parse(&format!("{}/payments", region.endpoint()))
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

    /// Retrieves a specific payment by its Lago ID
    ///
    /// # Arguments
    /// * `request` - The request containing the payment ID to retrieve
    ///
    /// # Returns
    /// A `Result` containing the payment data or an error
    pub async fn get_payment(&self, request: GetPaymentRequest) -> Result<GetPaymentResponse> {
        let region = self.config.region()?;
        let url = format!("{}/payments/{}", region.endpoint(), request.lago_id);
        self.make_request("GET", &url, None::<&()>).await
    }

    /// Creates a manual payment for an invoice
    ///
    /// # Arguments
    /// * `request` - The request containing the payment details
    ///
    /// # Returns
    /// A `Result` containing the created payment or an error
    pub async fn create_payment(
        &self,
        request: CreatePaymentRequest,
    ) -> Result<CreatePaymentResponse> {
        let region = self.config.region()?;
        let url = format!("{}/payments", region.endpoint());
        self.make_request("POST", &url, Some(&request)).await
    }

    /// Retrieves a list of payments for a specific customer
    ///
    /// # Arguments
    /// * `request` - The request containing the customer ID and optional filters
    ///
    /// # Returns
    /// A `Result` containing the list of payments or an error
    pub async fn list_customer_payments(
        &self,
        request: ListCustomerPaymentsRequest,
    ) -> Result<ListPaymentsResponse> {
        let region = self.config.region()?;
        let mut url = Url::parse(&format!(
            "{}/customers/{}/payments",
            region.endpoint(),
            urlencoding::encode(&request.external_customer_id)
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
