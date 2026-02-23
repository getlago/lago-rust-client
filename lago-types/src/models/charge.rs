use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a standalone charge filter response from the API.
///
/// This is used when charge filters are retrieved via standalone endpoints
/// (e.g., `/plans/{code}/charges/{charge_code}/filters`), which include
/// additional fields like `lago_id` and `created_at`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeFilterResponse {
    /// Unique identifier for the filter in Lago.
    pub lago_id: Uuid,
    /// Invoice display name for this filter.
    pub invoice_display_name: Option<String>,
    /// Filter properties.
    pub properties: Option<serde_json::Value>,
    /// Filter values mapping.
    pub values: Option<serde_json::Value>,
    /// When the filter was created.
    pub created_at: DateTime<Utc>,
}
