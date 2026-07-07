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
    /// Unique identifier for the event in Lago.
    ///
    /// This is an opaque identifier: a UUID for organizations on the Postgres
    /// events store, but a composite string on the ClickHouse events store.
    pub lago_id: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn event_json(lago_id: &str) -> String {
        format!(
            r#"{{
                "lago_id": "{lago_id}",
                "transaction_id": "event_1234",
                "lago_customer_id": null,
                "code": "bm_code",
                "timestamp": "2025-07-03T15:35:00Z",
                "lago_subscription_id": null,
                "external_subscription_id": "sub_1234",
                "created_at": "2025-07-03T15:35:22Z",
                "precise_total_amount_cents": null,
                "properties": null
            }}"#
        )
    }

    #[test]
    fn deserializes_uuid_lago_id() {
        let event: Event =
            serde_json::from_str(&event_json("1a901a90-1a90-1a90-1a90-1a901a901a90")).unwrap();
        assert_eq!(event.lago_id, "1a901a90-1a90-1a90-1a90-1a901a901a90");
    }

    #[test]
    fn deserializes_composite_lago_id_from_clickhouse_store() {
        let composite = "1a901a90-1a90-1a90-1a90-1a901a901a90-sub_1234-event_1234-1751549700";
        let event: Event = serde_json::from_str(&event_json(composite)).unwrap();
        assert_eq!(event.lago_id, composite);
    }
}
