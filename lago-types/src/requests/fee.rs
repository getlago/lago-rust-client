use crate::filters::{common::ListFilters, fee::FeeFilters};
use crate::models::PaginationParams;

/// Request parameters for listing fees from the `/fees` endpoint.
///
/// This struct combines pagination parameters and fee-specific filters
/// to build a comprehensive request for retrieving fee lists.
#[derive(Debug, Clone)]
pub struct ListFeesRequest {
    pub pagination: PaginationParams,
    pub filters: FeeFilters,
}

impl ListFeesRequest {
    /// Creates a new empty list fees request.
    ///
    /// # Returns
    /// A new `ListFeesRequest` instance with default pagination and no filters.
    pub fn new() -> Self {
        Self {
            pagination: PaginationParams::default(),
            filters: FeeFilters::default(),
        }
    }

    /// Sets the pagination parameters for the request.
    pub fn with_pagination(mut self, pagination: PaginationParams) -> Self {
        self.pagination = pagination;
        self
    }

    /// Sets the fee filters for the request.
    pub fn with_filters(mut self, filters: FeeFilters) -> Self {
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

impl Default for ListFeesRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Request parameters for retrieving a specific fee by its Lago ID.
#[derive(Debug, Clone)]
pub struct GetFeeRequest {
    pub fee_id: String,
}

impl GetFeeRequest {
    /// Creates a new get fee request.
    ///
    /// # Arguments
    /// * `fee_id` - The Lago ID (UUID) of the fee to retrieve
    ///
    /// # Returns
    /// A new `GetFeeRequest` instance with the specified fee ID.
    pub fn new(fee_id: String) -> Self {
        Self { fee_id }
    }
}
