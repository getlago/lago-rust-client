use serde::{Deserialize, Serialize};

use crate::filters::{common::ListFilters, date_range::DateRangeFilter};
use crate::models::ActivitySource;

/// Filter parameters for activity log list operations.
///
/// This struct represents the available filters that can be applied when
/// querying activity log lists from the API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActivityLogFilters {
    pub date_filter: DateRangeFilter,
    pub activity_types: Vec<String>,
    pub activity_sources: Vec<ActivitySource>,
    pub user_emails: Vec<String>,
    pub external_customer_id: Option<String>,
    pub external_subscription_id: Option<String>,
    pub resource_ids: Vec<String>,
    pub resource_types: Vec<String>,
}

impl ActivityLogFilters {
    /// Creates a new empty activity log filter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets both the start and end dates for the date filter range.
    pub fn with_date_range(mut self, from: String, to: String) -> Self {
        self.date_filter = self.date_filter.with_date_range(from, to);
        self
    }

    /// Sets the start date for the date filter.
    pub fn with_from_date(mut self, from: String) -> Self {
        self.date_filter = self.date_filter.with_from_date(from);
        self
    }

    /// Sets the end date for the date filter.
    pub fn with_to_date(mut self, to: String) -> Self {
        self.date_filter = self.date_filter.with_to_date(to);
        self
    }

    /// Sets the activity types filter.
    pub fn with_activity_types(mut self, types: Vec<String>) -> Self {
        self.activity_types = types;
        self
    }

    /// Sets the activity sources filter.
    pub fn with_activity_sources(mut self, sources: Vec<ActivitySource>) -> Self {
        self.activity_sources = sources;
        self
    }

    /// Sets the user emails filter.
    pub fn with_user_emails(mut self, emails: Vec<String>) -> Self {
        self.user_emails = emails;
        self
    }

    /// Sets the external customer ID filter.
    pub fn with_external_customer_id(mut self, customer_id: String) -> Self {
        self.external_customer_id = Some(customer_id);
        self
    }

    /// Sets the external subscription ID filter.
    pub fn with_external_subscription_id(mut self, subscription_id: String) -> Self {
        self.external_subscription_id = Some(subscription_id);
        self
    }

    /// Sets the resource IDs filter.
    pub fn with_resource_ids(mut self, ids: Vec<String>) -> Self {
        self.resource_ids = ids;
        self
    }

    /// Sets the resource types filter.
    pub fn with_resource_types(mut self, types: Vec<String>) -> Self {
        self.resource_types = types;
        self
    }
}

impl ListFilters for ActivityLogFilters {
    fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params: Vec<(&str, String)> = Vec::new();

        params.extend(self.date_filter.to_query_params());

        for activity_type in &self.activity_types {
            params.push(("activity_types[]", activity_type.clone()));
        }

        for source in &self.activity_sources {
            params.push(("activity_sources[]", format!("{source:?}").to_lowercase()));
        }

        for email in &self.user_emails {
            params.push(("user_emails[]", email.clone()));
        }

        if let Some(customer_id) = &self.external_customer_id {
            params.push(("external_customer_id", customer_id.clone()));
        }

        if let Some(subscription_id) = &self.external_subscription_id {
            params.push(("external_subscription_id", subscription_id.clone()));
        }

        for id in &self.resource_ids {
            params.push(("resource_ids[]", id.clone()));
        }

        for resource_type in &self.resource_types {
            params.push(("resource_types[]", resource_type.clone()));
        }

        params
    }
}
