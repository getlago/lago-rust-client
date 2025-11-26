use crate::models::PaginationParams;

use crate::filters::{activity_log::ActivityLogFilters, common::ListFilters};

/// Request parameters for listing activity logs.
#[derive(Debug, Clone)]
pub struct ListActivityLogsRequest {
    pub pagination: PaginationParams,
    pub filters: ActivityLogFilters,
}

impl ListActivityLogsRequest {
    /// Creates a new empty list activity logs request.
    pub fn new() -> Self {
        Self {
            pagination: PaginationParams::default(),
            filters: ActivityLogFilters::default(),
        }
    }

    /// Sets the pagination parameters for the request.
    pub fn with_pagination(mut self, pagination: PaginationParams) -> Self {
        self.pagination = pagination;
        self
    }

    /// Sets the activity log filters for the request.
    pub fn with_filters(mut self, filters: ActivityLogFilters) -> Self {
        self.filters = filters;
        self
    }

    /// Converts the request parameters into HTTP query parameters.
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = self.pagination.to_query_params();
        params.extend(self.filters.to_query_params());
        params
    }
}

impl Default for ListActivityLogsRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Request parameters for retrieving a specific activity log.
#[derive(Debug, Clone)]
pub struct GetActivityLogRequest {
    pub activity_id: String,
}

impl GetActivityLogRequest {
    /// Creates a new get activity log request.
    pub fn new(activity_id: String) -> Self {
        Self { activity_id }
    }
}
