use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::EnumString;
use uuid::Uuid;

/// Represents an activity log entry in the Lago system.
///
/// This struct contains all information about an action performed on
/// application resources, including the activity details and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityLogObject {
    pub activity_id: Uuid,
    pub activity_type: String,
    pub activity_source: ActivitySource,
    pub logged_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub user_email: Option<String>,
    pub resource_id: Option<Uuid>,
    pub resource_type: Option<String>,
    pub external_customer_id: Option<String>,
    pub external_subscription_id: Option<String>,
    pub activity_object: Option<Value>,
}

/// Source of the activity log entry.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ActivitySource {
    Api,
    Front,
    System,
}
