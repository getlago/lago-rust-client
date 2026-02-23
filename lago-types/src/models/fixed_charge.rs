use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use uuid::Uuid;

use crate::models::PlanTax;

/// Fixed charge model types.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FixedChargeModel {
    /// Standard pricing model.
    Standard,
    /// Graduated pricing model.
    Graduated,
    /// Volume pricing model.
    Volume,
}

/// Represents a fixed charge in the Lago billing system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedCharge {
    /// Unique identifier for the fixed charge in Lago.
    pub lago_id: Uuid,
    /// The Lago ID of the associated add-on.
    pub lago_add_on_id: Option<Uuid>,
    /// Display name for the fixed charge on invoices.
    pub invoice_display_name: Option<String>,
    /// The code of the associated add-on.
    pub add_on_code: Option<String>,
    /// When the fixed charge was created.
    pub created_at: DateTime<Utc>,
    /// Unique code for the fixed charge.
    pub code: Option<String>,
    /// The charge model to use.
    pub charge_model: FixedChargeModel,
    /// Whether the fixed charge is billed in advance.
    pub pay_in_advance: Option<bool>,
    /// Whether the fixed charge is prorated.
    pub prorated: Option<bool>,
    /// Fixed charge properties (model-specific configuration).
    pub properties: Option<serde_json::Value>,
    /// Number of units.
    pub units: Option<f64>,
    /// Parent fixed charge ID (for overrides).
    pub lago_parent_id: Option<Uuid>,
    /// Taxes applied to this fixed charge.
    pub taxes: Option<Vec<PlanTax>>,
}
