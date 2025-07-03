use crate::models::PaginationParams;

use crate::filters::{
    common::ListFilters,
    invoice::InvoiceFilters,
};

#[derive(Debug, Clone)]
pub struct ListInvoicesRequest {
    pub pagination: PaginationParams,
    pub filters: InvoiceFilters,
}

impl ListInvoicesRequest {
    pub fn new() -> Self {
        Self {
            pagination: PaginationParams::default(),
            filters: InvoiceFilters::default(),
        }
    }

    pub fn with_pagination(mut self, pagination: PaginationParams) -> Self {
        self.pagination = pagination;
        self
    }

    pub fn with_filters(mut self, filters: InvoiceFilters) -> Self {
        self.filters = filters;
        self
    }

    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = self.pagination.to_query_params();
        params.extend(self.filters.to_query_params());
        params
    }
}

impl Default for ListInvoicesRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct GetInvoiceRequest {
    pub invoice_id: String,
}

impl GetInvoiceRequest {
    pub fn new(invoice_id: String) -> Self {
        Self { invoice_id }
    }
}