use serde::{Serialize, Deserialize};

use crate::filters::common::ListFilters;


/// Filter parameters for date range operations.
/// 
/// This struct represents date range filters that can be applied when
/// querying resources from the API within a specific time period.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DateRangeFilter {
    pub from_date: Option<String>,
    pub to_date: Option<String>,
}

impl DateRangeFilter {
    /// Create a new empty date range filter.
    /// 
    /// # Returns
    /// A new `DateRangeFilter` instance with no date filters set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets both the start and end dates for the filter range.
    /// 
    /// # Arguments
    /// * `from` - The start date string
    /// * `to` - The end date string
    /// 
    /// # Returns
    /// The modifier filter instance for method chaining.
    pub fn with_date_range(mut self, from: String, to: String) -> Self {
        self.from_date = Some(from);
        self.to_date = Some(to);
        self
    }

    /// Sets the start date for the filter range.
    /// 
    /// # Arguments
    /// * `from`- The start date string
    /// 
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_from_date(mut self, from: String) -> Self {
        self.from_date = Some(from);
        self
    }

    /// Sets the end date for the filter range.
    /// 
    /// # Arguments
    /// * `to` - The end date string
    /// 
    /// # Returns
    /// The modified filter intance for method chaining.
    pub fn with_to_date(mut self, to: String) -> Self {
        self.to_date = Some(to);
        self
    }
}


impl ListFilters for DateRangeFilter {
    /// Converts the date range filter into HTTP query parameters.
    /// 
    /// # Returns
    /// A vector of query parameter tuples containing the date range criteria.
    fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();

        if let Some(from) = &self.from_date {
            params.push(("from_date", from.clone()));
        }

        if let Some(to) = &self.to_date {
            params.push(("to_date", to.clone()));
        }

        params
    }
}