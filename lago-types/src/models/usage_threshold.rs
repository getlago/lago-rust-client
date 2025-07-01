use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageThreshold {
  pub lago_id: Uuid,
  pub amount_cents: i64,
  pub recurring: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub threshold_display_name: Option<String>,
}