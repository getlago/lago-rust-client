use serde::{Serialize, Deserialize};

use crate::filters::common::ListFilters;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DateRangeFilter {
    pub from_date: Option<String>,
    pub to_date: Option<String>,
}

impl DateRangeFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_date_range(mut self, from: String, to: String) -> Self {
        self.from_date = Some(from);
        self.to_date = Some(to);
        self
    }

    pub fn with_from_date(mut self, from: String) -> Self {
        self.from_date = Some(from);
        self
    }

    pub fn with_to_date(mut self, to: String) -> Self {
        self.to_date = Some(to);
        self
    }
}


impl ListFilters for DateRangeFilter {
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