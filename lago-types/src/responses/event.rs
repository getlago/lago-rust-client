use serde::{Deserialize, Serialize};

use crate::models::Event;

/// Response for retrieving a single event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEventResponse {
    pub event: Event,
}

/// Response for creating an event.
///
/// Note: The create event endpoint returns a minimal response with just the
/// event wrapper, as events are processed asynchronously.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEventResponse {
    pub event: EventCreated,
}

/// The created event acknowledgment.
///
/// When an event is created, Lago returns a minimal response containing
/// the transaction_id, external_customer_id, and code to confirm receipt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCreated {
    /// The transaction ID of the created event
    pub transaction_id: String,
    /// The external customer ID (if provided)
    pub external_customer_id: Option<String>,
    /// The external subscription ID (if provided)
    pub external_subscription_id: Option<String>,
    /// The billable metric code
    pub code: String,
}
