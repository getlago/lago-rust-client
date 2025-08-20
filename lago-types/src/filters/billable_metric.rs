use serde::{Deserialize, Serialize};

use crate::filters::common::ListFilters;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BillableMetricFilter {
    pub aggregation_type: Option<String>,
    pub recurring: Option<bool>,
}

impl BillableMetricFilter {
    /// Create a new empty billable metric filter.
    ///
    /// # Returns
    /// A new `BillableMetricFilter` instance with no filters set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the aggregation type filter.
    ///
    /// # Arguments
    /// * `aggregation_type` - The aggregation type to filter by
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_aggregation_type(mut self, aggregation_type: String) -> Self {
        self.aggregation_type = Some(aggregation_type);
        self
    }

    /// Sets the recurring filter.
    ///
    /// # Arguments
    /// * `recurring` - Whether to filter by recurring billable metrics
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_recurring(mut self, recurring: bool) -> Self {
        self.recurring = Some(recurring);
        self
    }
}

impl ListFilters for BillableMetricFilter {
    /// Converts the billable metric filter into HTTP query parameters.
    ///
    /// # Returns
    /// A vector of query parameters containing the filter criteria.
    fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params: Vec<(&'static str, String)> = Vec::new();

        if let Some(aggregation_type) = &self.aggregation_type {
            params.push(("aggregation_type", aggregation_type.clone()));
        }

        if let Some(recurring) = &self.recurring {
            params.push(("recurring", recurring.to_string()));
        }

        params
    }
}
