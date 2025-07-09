use serde::{Deserialize, Serialize};

use crate::filters::common::ListFilters;

/// Filter parameters for customer list operations.
/// 
/// This struct represents the available filters that can be applied when
/// querying customer lists from the API.

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomerFilter {
    pub external_customer_id: Option<String>,
}

impl CustomerFilter {
    /// Create a new empty customer filter.
    /// 
    /// # Returns
    /// A new `CustomerFilter` instance with no filters set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the external customer ID filter.
    /// 
    /// # Arguments
    /// * `customer_id` - The external customer ID to filter by.
    /// 
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_customer_id(mut self, customer_id: String) -> Self {
        self.external_customer_id = Some(customer_id);
        self
    }
}

impl ListFilters for CustomerFilter {
    /// Convers the customer filter into HTTP query parameters.
    /// 
    /// # Returns
    /// A vector of query parameter containing the filter criteria.
    fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params: Vec<(&'static str, String)> = Vec::new();

        if let Some(customer_id) = &self.external_customer_id {
            params.push(("external_customer_id", customer_id.clone()));
        }

        params
    }
}