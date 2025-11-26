use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Map;
use strum_macros::EnumString;
use uuid::Uuid;

/// Represents an API log entry in the Lago system.
///
/// This struct contains all information about an API request made to Lago's API,
/// including the request details, response information, and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiLogObject {
    pub request_id: Uuid,
    pub api_version: String,
    pub client: String,
    pub http_method: HttpMethod,
    pub http_status: i32,
    pub request_origin: String,
    pub request_path: String,
    pub request_body: Map<String, serde_json::Value>,
    pub request_response: Option<Map<String, serde_json::Value>>,
    pub logged_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// HTTP methods supported by the API logs.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum HttpMethod {
    Post,
    Put,
    Delete,
}

/// HTTP status filter types for API logs.
///
/// This enum represents the different ways to filter API logs by status:
/// either by specific HTTP status codes or by request outcome (succeeded/failed).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HttpStatus {
    Code(i32),
    Outcome(StatusOutcome),
}

/// Request outcome status for filtering.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum StatusOutcome {
    Succeeded,
    Failed,
}
