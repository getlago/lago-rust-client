use lago_types::{
    error::{LagoError, Result},
    requests::invoice::{
        CreateInvoiceRequest, DownloadInvoiceRequest, GetInvoiceRequest, InvoicePreviewRequest,
        ListCustomerInvoicesRequest, ListInvoicesRequest, RefreshInvoiceRequest,
        RetryInvoicePaymentRequest, RetryInvoiceRequest, UpdateInvoiceRequest,
    },
    responses::invoice::{
        CreateInvoiceResponse, DownloadInvoiceResponse, GetInvoiceResponse, InvoicePreviewResponse,
        ListInvoicesResponse, RefreshInvoiceResponse, RetryInvoicePaymentResponse,
        RetryInvoiceResponse, UpdateInvoiceResponse,
    },
};
use url::Url;

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
    pub async fn list_invoices(
        &self,
        request: Option<ListInvoicesRequest>,
    ) -> Result<ListInvoicesResponse> {
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

    /// Previews an invoice without creating it
    ///
    /// This endpoint allows you to retrieve an estimated invoice before finalization.
    /// It can be used to preview invoices for new subscriptions or existing customers.
    ///
    /// # Arguments
    /// * `request` - The invoice preview request containing customer and subscription details
    ///
    /// # Returns
    /// A `Result` containing the previewed invoice or an error
    pub async fn preview_invoice(
        &self,
        request: InvoicePreviewRequest,
    ) -> Result<InvoicePreviewResponse> {
        let region = self.config.region()?;
        let url = format!("{}/invoices/preview", region.endpoint());
        self.make_request("POST", &url, Some(&request)).await
    }

    /// Creates a one-off invoice for a customer
    ///
    /// This endpoint allows you to create a one-off invoice with add-on charges
    /// for a specific customer.
    ///
    /// # Arguments
    /// * `request` - The request containing the invoice details and fees
    ///
    /// # Returns
    /// A `Result` containing the created invoice or an error
    pub async fn create_invoice(
        &self,
        request: CreateInvoiceRequest,
    ) -> Result<CreateInvoiceResponse> {
        let region = self.config.region()?;
        let url = format!("{}/invoices", region.endpoint());
        self.make_request("POST", &url, Some(&request)).await
    }

    /// Updates an existing invoice
    ///
    /// This endpoint allows you to update the payment status or metadata
    /// of an existing invoice.
    ///
    /// # Arguments
    /// * `request` - The request containing the invoice ID and update data
    ///
    /// # Returns
    /// A `Result` containing the updated invoice or an error
    pub async fn update_invoice(
        &self,
        request: UpdateInvoiceRequest,
    ) -> Result<UpdateInvoiceResponse> {
        let region = self.config.region()?;
        let url = format!("{}/invoices/{}", region.endpoint(), request.lago_id);
        self.make_request("PUT", &url, Some(&request)).await
    }

    /// Retrieves a list of invoices for a specific customer
    ///
    /// # Arguments
    /// * `request` - The request containing the customer ID and optional filters
    ///
    /// # Returns
    /// A `Result` containing the list of invoices or an error
    pub async fn list_customer_invoices(
        &self,
        request: ListCustomerInvoicesRequest,
    ) -> Result<ListInvoicesResponse> {
        let region = self.config.region()?;
        let mut url = Url::parse(&format!(
            "{}/customers/{}/invoices",
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

    /// Refreshes a draft invoice
    ///
    /// This endpoint re-fetches the customer information and recomputes the taxes
    /// for a draft invoice. Only draft invoices can be refreshed.
    ///
    /// # Arguments
    /// * `request` - The request containing the invoice ID to refresh
    ///
    /// # Returns
    /// A `Result` containing the refreshed invoice or an error
    pub async fn refresh_invoice(
        &self,
        request: RefreshInvoiceRequest,
    ) -> Result<RefreshInvoiceResponse> {
        let region = self.config.region()?;
        let url = format!("{}/invoices/{}/refresh", region.endpoint(), request.lago_id);
        self.make_request("PUT", &url, None::<&()>).await
    }

    /// Downloads an invoice PDF
    ///
    /// This endpoint triggers the generation of the invoice PDF if not already
    /// generated, and returns the invoice with a file_url field containing
    /// the URL to download the PDF.
    ///
    /// # Arguments
    /// * `request` - The request containing the invoice ID to download
    ///
    /// # Returns
    /// A `Result` containing the invoice with file_url or an error
    pub async fn download_invoice(
        &self,
        request: DownloadInvoiceRequest,
    ) -> Result<DownloadInvoiceResponse> {
        let region = self.config.region()?;
        let url = format!(
            "{}/invoices/{}/download",
            region.endpoint(),
            request.lago_id
        );
        self.make_request("POST", &url, None::<&()>).await
    }

    /// Retries a failed invoice finalization
    ///
    /// This endpoint retries the finalization process for invoices that
    /// failed during generation. Only failed invoices can be retried.
    ///
    /// # Arguments
    /// * `request` - The request containing the invoice ID to retry
    ///
    /// # Returns
    /// A `Result` containing the retried invoice or an error
    pub async fn retry_invoice(
        &self,
        request: RetryInvoiceRequest,
    ) -> Result<RetryInvoiceResponse> {
        let region = self.config.region()?;
        let url = format!("{}/invoices/{}/retry", region.endpoint(), request.lago_id);
        self.make_request("POST", &url, None::<&()>).await
    }

    /// Retries a failed invoice payment
    ///
    /// This endpoint resends the invoice for collection and retries the payment
    /// with the payment provider. Only invoices with failed payment status can
    /// be retried.
    ///
    /// # Arguments
    /// * `request` - The request containing the invoice ID to retry payment for
    ///
    /// # Returns
    /// A `Result` containing the invoice or an error
    pub async fn retry_invoice_payment(
        &self,
        request: RetryInvoicePaymentRequest,
    ) -> Result<RetryInvoicePaymentResponse> {
        let region = self.config.region()?;
        let url = format!(
            "{}/invoices/{}/retry_payment",
            region.endpoint(),
            request.lago_id
        );
        self.make_request("POST", &url, None::<&()>).await
    }
}
