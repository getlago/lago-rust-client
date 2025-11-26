use serde::{Deserialize, Serialize};

use crate::filters::{common::ListFilters, date_range::DateRangeFilter};
use crate::models::{HttpMethod, HttpStatus};

/// Filter parameters for API log list operations.
///
/// This struct represents the available filters that can be applied when
/// querying API log lists from the API, combining date, HTTP method,
/// status, version, and path filters.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiLogFilters {
    pub date_filter: DateRangeFilter,
    pub http_methods: Vec<HttpMethod>,
    pub http_statuses: Vec<HttpStatus>,
    pub api_version: Option<String>,
    pub request_paths: Option<String>,
}

impl ApiLogFilters {
    /// Creates a new empty API log filter.
    ///
    /// # Returns
    /// A new `ApiLogFilters` instance with no filters set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets both the start and end dates for the date filter range.
    ///
    /// # Arguments
    /// * `from` - The start date string for API logs
    /// * `to` - The end date string for API logs
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_date_range(mut self, from: String, to: String) -> Self {
        self.date_filter = self.date_filter.with_date_range(from, to);
        self
    }

    /// Sets the start date for the date filter.
    ///
    /// # Arguments
    /// * `from` - The start date string for API logs
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_from_date(mut self, from: String) -> Self {
        self.date_filter = self.date_filter.with_from_date(from);
        self
    }

    /// Sets the end date for the date filter.
    ///
    /// # Arguments
    /// * `to` - The end date string for API logs
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_to_date(mut self, to: String) -> Self {
        self.date_filter = self.date_filter.with_to_date(to);
        self
    }

    /// Sets the HTTP methods filter.
    ///
    /// # Arguments
    /// * `methods` - A vector of HTTP methods to filter by
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_http_methods(mut self, methods: Vec<HttpMethod>) -> Self {
        self.http_methods = methods;
        self
    }

    /// Sets the HTTP statuses filter.
    ///
    /// # Arguments
    /// * `statuses` - A vector of HTTP statuses to filter by
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_http_statuses(mut self, statuses: Vec<HttpStatus>) -> Self {
        self.http_statuses = statuses;
        self
    }

    /// Sets the API version filter.
    ///
    /// # Arguments
    /// * `version` - The API version to filter by
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_api_version(mut self, version: String) -> Self {
        self.api_version = Some(version);
        self
    }

    /// Sets the request paths filter.
    ///
    /// # Arguments
    /// * `paths` - The request path to filter by
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_request_paths(mut self, paths: String) -> Self {
        self.request_paths = Some(paths);
        self
    }
}

impl ListFilters for ApiLogFilters {
    /// Converts the API log filters into HTTP query parameters.
    ///
    /// This method combines all the individual filter criteria into a single
    /// vector of query parameters.
    ///
    /// # Returns
    /// A vector of query parameter tuples containing all the filter criteria.
    fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params: Vec<(&str, String)> = Vec::new();

        params.extend(self.date_filter.to_query_params());

        for method in &self.http_methods {
            params.push(("http_methods[]", format!("{method:?}").to_lowercase()));
        }

        for status in &self.http_statuses {
            let status_str = match status {
                HttpStatus::Code(code) => code.to_string(),
                HttpStatus::Outcome(outcome) => format!("{outcome:?}").to_lowercase(),
            };
            params.push(("http_statuses[]", status_str));
        }

        if let Some(version) = &self.api_version {
            params.push(("api_version", version.clone()));
        }

        if let Some(paths) = &self.request_paths {
            params.push(("request_paths", paths.clone()));
        }

        params
    }
}
