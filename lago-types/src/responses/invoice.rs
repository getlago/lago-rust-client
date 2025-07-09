use serde::{Deserialize, Serialize};
use crate::models::{Invoice, PaginationMeta};

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