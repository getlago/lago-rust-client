use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::PaginationParams;

/// Request parameters for listing events.
///
/// This struct combines pagination parameters and optional filters
/// to build a comprehensive request for retrieving event lists.
#[derive(Debug, Clone, Default)]
pub struct ListEventsRequest {
    /// Pagination parameters.
    pub pagination: PaginationParams,
    /// Filter by external subscription ID.
    pub external_subscription_id: Option<String>,
    /// Filter by billable metric code.
    pub code: Option<String>,
    /// Requires `external_subscription_id` to be set.
    /// Filter events by timestamp after the subscription started at datetime.
    pub timestamp_from_started_at: Option<bool>,
    /// Filter events by timestamp starting from a specific date (ISO 8601 format).
    pub timestamp_from: Option<String>,
    /// Filter events by timestamp up to a specific date (ISO 8601 format).
    pub timestamp_to: Option<String>,
}

impl ListEventsRequest {
    /// Creates a new empty list events request.
    ///
    /// # Returns
    /// A new `ListEventsRequest` instance with default pagination and no filters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the pagination parameters for the request.
    ///
    /// # Arguments
    /// * `pagination` - The pagination parameters to use
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_pagination(mut self, pagination: PaginationParams) -> Self {
        self.pagination = pagination;
        self
    }

    /// Sets the external subscription ID filter.
    ///
    /// # Arguments
    /// * `external_subscription_id` - The external subscription ID to filter by
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_external_subscription_id(mut self, external_subscription_id: String) -> Self {
        self.external_subscription_id = Some(external_subscription_id);
        self
    }

    /// Sets the billable metric code filter.
    ///
    /// # Arguments
    /// * `code` - The billable metric code to filter by
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    /// Sets whether to filter events by timestamp after the subscription started at datetime.
    /// Requires `external_subscription_id` to be set.
    ///
    /// # Arguments
    /// * `timestamp_from_started_at` - Whether to filter from subscription start
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_timestamp_from_started_at(mut self, timestamp_from_started_at: bool) -> Self {
        self.timestamp_from_started_at = Some(timestamp_from_started_at);
        self
    }

    /// Sets the timestamp from filter (events with timestamp >= this date).
    ///
    /// # Arguments
    /// * `timestamp_from` - The start date in ISO 8601 format
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_timestamp_from(mut self, timestamp_from: String) -> Self {
        self.timestamp_from = Some(timestamp_from);
        self
    }

    /// Sets the timestamp to filter (events with timestamp <= this date).
    ///
    /// # Arguments
    /// * `timestamp_to` - The end date in ISO 8601 format
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_timestamp_to(mut self, timestamp_to: String) -> Self {
        self.timestamp_to = Some(timestamp_to);
        self
    }

    /// Sets the timestamp range filter.
    ///
    /// # Arguments
    /// * `from` - The start date in ISO 8601 format
    /// * `to` - The end date in ISO 8601 format
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_timestamp_range(mut self, from: String, to: String) -> Self {
        self.timestamp_from = Some(from);
        self.timestamp_to = Some(to);
        self
    }

    /// Converts the request parameters into HTTP query parameters.
    ///
    /// # Returns
    /// A vector of query parameter tuples containing both pagination and filter criteria.
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = self.pagination.to_query_params();

        if let Some(external_subscription_id) = &self.external_subscription_id {
            params.push(("external_subscription_id", external_subscription_id.clone()));
        }

        if let Some(code) = &self.code {
            params.push(("code", code.clone()));
        }

        if let Some(timestamp_from_started_at) = &self.timestamp_from_started_at {
            params.push((
                "timestamp_from_started_at",
                timestamp_from_started_at.to_string(),
            ));
        }

        if let Some(timestamp_from) = &self.timestamp_from {
            params.push(("timestamp_from", timestamp_from.clone()));
        }

        if let Some(timestamp_to) = &self.timestamp_to {
            params.push(("timestamp_to", timestamp_to.clone()));
        }

        params
    }
}

/// Request to retrieve a specific event by transaction ID.
#[derive(Debug, Clone)]
pub struct GetEventRequest {
    /// The transaction ID of the event to retrieve (must be URL encoded)
    pub transaction_id: String,
}

impl GetEventRequest {
    /// Creates a new get event request.
    ///
    /// # Arguments
    /// * `transaction_id` - The transaction ID of the event to retrieve
    ///
    /// # Returns
    /// A new `GetEventRequest` instance
    pub fn new(transaction_id: String) -> Self {
        Self { transaction_id }
    }
}

/// Input data for creating a usage event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEventInput {
    /// Unique identifier for this event (used for idempotency and retrieval)
    pub transaction_id: String,
    /// External customer ID - required if external_subscription_id is not provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_customer_id: Option<String>,
    /// External subscription ID - required if external_customer_id is not provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_subscription_id: Option<String>,
    /// Billable metric code
    pub code: String,
    /// Event timestamp (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Custom properties/metadata for the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Value>,
    /// Precise total amount in cents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precise_total_amount_cents: Option<i64>,
}

impl CreateEventInput {
    /// Creates a new event input for a customer.
    ///
    /// # Arguments
    /// * `transaction_id` - Unique identifier for the event
    /// * `external_customer_id` - The external ID of the customer
    /// * `code` - The billable metric code
    ///
    /// # Returns
    /// A new `CreateEventInput` instance
    pub fn for_customer(
        transaction_id: String,
        external_customer_id: String,
        code: String,
    ) -> Self {
        Self {
            transaction_id,
            external_customer_id: Some(external_customer_id),
            external_subscription_id: None,
            code,
            timestamp: None,
            properties: None,
            precise_total_amount_cents: None,
        }
    }

    /// Creates a new event input for a subscription.
    ///
    /// # Arguments
    /// * `transaction_id` - Unique identifier for the event
    /// * `external_subscription_id` - The external ID of the subscription
    /// * `code` - The billable metric code
    ///
    /// # Returns
    /// A new `CreateEventInput` instance
    pub fn for_subscription(
        transaction_id: String,
        external_subscription_id: String,
        code: String,
    ) -> Self {
        Self {
            transaction_id,
            external_customer_id: None,
            external_subscription_id: Some(external_subscription_id),
            code,
            timestamp: None,
            properties: None,
            precise_total_amount_cents: None,
        }
    }

    /// Sets the timestamp for the event.
    ///
    /// # Arguments
    /// * `timestamp` - Unix timestamp in seconds
    ///
    /// # Returns
    /// The modified input instance for method chaining
    pub fn with_timestamp(mut self, timestamp: i64) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    /// Sets custom properties for the event.
    ///
    /// # Arguments
    /// * `properties` - JSON object containing event properties
    ///
    /// # Returns
    /// The modified input instance for method chaining
    pub fn with_properties(mut self, properties: Value) -> Self {
        self.properties = Some(properties);
        self
    }

    /// Sets the precise total amount in cents.
    ///
    /// # Arguments
    /// * `amount` - The precise amount in cents
    ///
    /// # Returns
    /// The modified input instance for method chaining
    pub fn with_precise_total_amount_cents(mut self, amount: i64) -> Self {
        self.precise_total_amount_cents = Some(amount);
        self
    }
}

/// Request wrapper for creating a usage event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEventRequest {
    /// The event input data
    pub event: CreateEventInput,
}

impl CreateEventRequest {
    /// Creates a new create event request.
    ///
    /// # Arguments
    /// * `event` - The event input data
    ///
    /// # Returns
    /// A new `CreateEventRequest` instance
    pub fn new(event: CreateEventInput) -> Self {
        Self { event }
    }
}
