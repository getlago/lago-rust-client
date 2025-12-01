use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    /// Event timestamp (Unix timestamp or ISO 8601 string)
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
    pub fn for_customer(transaction_id: String, external_customer_id: String, code: String) -> Self {
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
