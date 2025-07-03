use serde::{Deserialize, Serialize};

use crate::filters::common::ListFilters;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomerFilter {
    pub external_customer_id: Option<String>,
}

impl CustomerFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_customer_id(mut self, customer_id: String) -> Self {
        self.external_customer_id = Some(customer_id);
        self
    }
}

impl ListFilters for CustomerFilter {
    fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params: Vec<(&'static str, String)> = Vec::new();

        if let Some(customer_id) = &self.external_customer_id {
            params.push(("external_customer_id", customer_id.clone()));
        }

        params
    }
}