use crate::models::{Invoice, PaginationMeta};
use serde::{Deserialize, Serialize};

/// Response containing a list of invoices with pagination metadata.
///
/// This struct represents the API response for invoice listing requests,
/// including both the invoice data and pagination information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListInvoicesResponse {
    pub invoices: Vec<Invoice>,
    pub meta: PaginationMeta,
}

/// Response containing a single invoice.
///
/// This struct represents the API response for retrieving a specific
/// invoice by its identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInvoiceResponse {
    pub invoice: Invoice,
}

/// Response containing a previewed invoice.
///
/// This struct represents the API response for the invoice preview endpoint.
/// The response contains the same Invoice structure as other invoice endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicePreviewResponse {
    pub invoice: Invoice,
}

/// Response containing a created invoice.
///
/// This struct represents the API response for the create invoice endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoiceResponse {
    pub invoice: Invoice,
}

/// Response containing an updated invoice.
///
/// This struct represents the API response for the update invoice endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInvoiceResponse {
    pub invoice: Invoice,
}

/// Response containing a refreshed invoice.
///
/// This struct represents the API response for the refresh invoice endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshInvoiceResponse {
    pub invoice: Invoice,
}

/// Response containing a downloaded invoice.
///
/// This struct represents the API response for the download invoice endpoint.
/// The response includes the invoice with a file_url field containing the PDF URL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadInvoiceResponse {
    pub invoice: Invoice,
}

/// Response for retrying an invoice finalization.
///
/// This struct represents the API response for the retry invoice endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryInvoiceResponse {
    pub invoice: Invoice,
}

/// Response for retrying an invoice payment.
///
/// This struct represents the API response for the retry payment endpoint.
/// Note: The Lago API returns an empty body (HTTP 200 with no content) for this endpoint.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RetryInvoicePaymentResponse {}

/// Response for voiding an invoice.
///
/// This struct represents the API response for the void invoice endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoidInvoiceResponse {
    pub invoice: Invoice,
}
