use crate::models::PaginationParams;

use crate::filters::{api_log::ApiLogFilters, common::ListFilters};

/// Request parameters for listing API logs.
///
/// This struct combines pagination parameters and API log-specific filters
/// to build a comprehensive request for retrieving API log lists.
#[derive(Debug, Clone)]
pub struct ListApiLogsRequest {
    pub pagination: PaginationParams,
    pub filters: ApiLogFilters,
}

impl ListApiLogsRequest {
    /// Creates a new empty list API logs request.
    ///
    /// # Returns
    /// A new `ListApiLogsRequest` instance with default pagination and no filters.
    pub fn new() -> Self {
        Self {
            pagination: PaginationParams::default(),
            filters: ApiLogFilters::default(),
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

    /// Sets the API log filters for the request.
    ///
    /// # Arguments
    /// * `filters` - The API log filters to apply
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_filters(mut self, filters: ApiLogFilters) -> Self {
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

impl Default for ListApiLogsRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Request parameters for retrieving a specific API log.
///
/// This struct contains the identifier needed to fetch a single API log
/// from the API.
#[derive(Debug, Clone)]
pub struct GetApiLogRequest {
    pub request_id: String,
}

impl GetApiLogRequest {
    /// Creates a new get API log request.
    ///
    /// # Arguments
    /// * `request_id` - The unique identifier of the API log to retrieve
    ///
    /// # Returns
    /// A new `GetApiLogRequest` instance with the specified request ID.
    pub fn new(request_id: String) -> Self {
        Self { request_id }
    }
}
