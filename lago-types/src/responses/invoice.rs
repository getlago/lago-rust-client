use serde::{Deserialize, Serialize};
use crate::models::{Invoice, PaginationMeta};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListInvoicesResponse {
    pub invoices: Vec<Invoice>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInvoiceResponse {
    pub invoice: Invoice,
}