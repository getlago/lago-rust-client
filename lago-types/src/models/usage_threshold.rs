use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a usage threshold in the Lago billing system.
///
/// Usage thresholds define spending limits that can trigger notifications
/// or actions when usage amounts exceed specified thresholds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageThreshold {
    pub lago_id: Uuid,
    pub amount_cents: i64,
    pub recurring: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub threshold_display_name: Option<String>,
}
