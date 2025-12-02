use serde::{Deserialize, Serialize};

use crate::filters::common::ListFilters;

/// Filter parameters for plan list operations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlanFilters {
    // Currently no specific filters for plans, but this struct
    // is created for consistency and future extensibility.
}

impl PlanFilters {
    /// Creates a new empty plan filter.
    pub fn new() -> Self {
        Self::default()
    }
}

impl ListFilters for PlanFilters {
    fn to_query_params(&self) -> Vec<(&str, String)> {
        // No specific filters for plans currently
        Vec::new()
    }
}
