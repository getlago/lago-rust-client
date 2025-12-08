use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Represents a usage measurement event in the Lago billing system.
///
/// Events are used to track customer usage and are aggregated into invoice
/// line items based on billable metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Unique identifier for the event in Lago
    pub lago_id: Uuid,
    /// External transaction reference provided when creating the event
    pub transaction_id: String,
    /// Lago ID of the associated customer (may be null)
    pub lago_customer_id: Option<Uuid>,
    /// Billable metric code
    pub code: String,
    /// Event occurrence timestamp
    pub timestamp: DateTime<Utc>,
    /// Lago ID of the linked subscription (may be null)
    pub lago_subscription_id: Option<Uuid>,
    /// External subscription reference
    pub external_subscription_id: Option<String>,
    /// Record creation timestamp
    pub created_at: DateTime<Utc>,
    /// Precise amount calculation in cents
    pub precise_total_amount_cents: Option<String>,
    /// Custom event metadata/properties
    pub properties: Option<Value>,
}
