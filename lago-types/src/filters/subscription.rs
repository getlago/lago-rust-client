use serde::{Deserialize, Serialize};

use crate::filters::common::ListFilters;
use crate::models::SubscriptionStatus;

/// Filter parameters for subscription list operations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubscriptionFilters {
    /// Filter by plan code.
    pub plan_code: Option<String>,
    /// Filter by subscription status(es).
    pub status: Option<Vec<SubscriptionStatus>>,
}

impl SubscriptionFilters {
    /// Creates a new empty subscription filter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the plan code filter.
    pub fn with_plan_code(mut self, plan_code: String) -> Self {
        self.plan_code = Some(plan_code);
        self
    }

    /// Sets a single status filter.
    pub fn with_status(mut self, status: SubscriptionStatus) -> Self {
        self.status = Some(vec![status]);
        self
    }

    /// Sets multiple status filters.
    pub fn with_statuses(mut self, statuses: Vec<SubscriptionStatus>) -> Self {
        self.status = Some(statuses);
        self
    }
}

impl ListFilters for SubscriptionFilters {
    fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(plan_code) = &self.plan_code {
            params.push(("plan_code", plan_code.clone()));
        }

        if let Some(statuses) = &self.status {
            for status in statuses {
                params.push(("status[]", format!("{status:?}").to_lowercase()));
            }
        }

        params
    }
}
