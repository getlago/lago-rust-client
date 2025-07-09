use url::Url;
use lago_types::{
    error::{LagoError, Result},
    requests::invoice::{ListInvoicesRequest, GetInvoiceRequest},
    responses::invoice::{ListInvoicesResponse, GetInvoiceResponse},
};

use crate::client::LagoClient;

/// Invoice-related operations for the Lago client
impl LagoClient {
    /// Retrieves a list of invoices with optional filtering parameters
    /// 
    /// # Arguments
    /// * `request` - Optional filtering parameters for the invoice list
    /// 
    /// # Returns
    /// A `Result` containing the list of invoices or an error
    pub async fn list_invoices(&self, request: Option<ListInvoicesRequest>) -> Result<ListInvoicesResponse> {
        let request = request.unwrap_or_default();
        let region = self.config.region()?;
        let mut url = Url::parse(&format!("{}/invoices", region.endpoint()))
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

    /// Retrieves a specific invoice by its ID
    /// 
    /// # Arguments
    /// * `request` - The request containing the invoice ID to retrieve
    /// 
    /// # Returns
    /// A `Result` containing the invoice data or an error
    pub async fn get_invoice(&self, request: GetInvoiceRequest) -> Result<GetInvoiceResponse> {
        let region = self.config.region()?;
        let url = format!("{}/invoices/{}", region.endpoint(), request.invoice_id);
        self.make_request("GET", &url, None::<&()>).await
    }
}