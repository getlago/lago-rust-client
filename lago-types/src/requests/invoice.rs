use crate::models::PaginationParams;

use crate::filters::{
    common::ListFilters,
    invoice::InvoiceFilters,
};

/// Request parameters for listing invoices.
/// 
/// This struct combines pagination parameters and invoice-specific filters
/// to build a comprehensive request for retrieving invoice lists.
#[derive(Debug, Clone)]
pub struct ListInvoicesRequest {
    pub pagination: PaginationParams,
    pub filters: InvoiceFilters,
}

impl ListInvoicesRequest {
    /// Creates a new empty list invoices request.
    /// 
    /// # Returns
    /// A new `ListInvoicesRequest` instance with default pagination and no filters.
    pub fn new() -> Self {
        Self {
            pagination: PaginationParams::default(),
            filters: InvoiceFilters::default(),
        }
    }

    /// Sets the pagination parameters for the request.
    /// 
    /// # Arguments
    /// * `pagination` - The pagination parameters to use
    /// 
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_pagination(mut self, pagination: PaginationParams) -> Self {
        self.pagination = pagination;
        self
    }

    /// Sets the invoice filters for the request.
    /// 
    /// # Arguments
    /// * `filters` - The invoice filters to apply
    /// 
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_filters(mut self, filters: InvoiceFilters) -> Self {
        self.filters = filters;
        self
    }

    /// Converts the request parameters into HTTP query parameters.
    /// 
    /// # Returns
    /// A vector of query parameter tuples containing both pagination and filter criteria.
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

/// Request parameters for retrieving a specific invoice.
/// 
/// This struct contains the identifier needed to fetch a single invoice
/// from the API.
#[derive(Debug, Clone)]
pub struct GetInvoiceRequest {
    pub invoice_id: String,
}

impl GetInvoiceRequest {
    /// Creates a new get invoice request.
    /// 
    /// # Arguments
    /// * `invoice_id` - The unique identifier of the invoice to retrieve
    /// 
    /// # Returns
    /// A new `GetInvoiceRequest` instance with the specified invoice ID.
    pub fn new(invoice_id: String) -> Self {
        Self { invoice_id }
    }
}