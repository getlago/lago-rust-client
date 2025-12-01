use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use uuid::Uuid;

use crate::models::UsageThreshold;

/// Represents a plan in the Lago billing system.
///
/// A plan defines pricing configuration that can be assigned to customers
/// through subscriptions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    /// Unique identifier for the plan in Lago.
    pub lago_id: Uuid,
    /// Name of the plan.
    pub name: String,
    /// Display name for invoices.
    pub invoice_display_name: Option<String>,
    /// When the plan was created.
    pub created_at: DateTime<Utc>,
    /// Unique code for the plan.
    pub code: String,
    /// Billing interval (weekly, monthly, quarterly, yearly).
    pub interval: PlanInterval,
    /// Description of the plan.
    pub description: Option<String>,
    /// Base amount in cents.
    pub amount_cents: i64,
    /// Currency for the amount.
    pub amount_currency: String,
    /// Trial period in days.
    pub trial_period: Option<f64>,
    /// Whether the plan is billed in advance.
    pub pay_in_advance: bool,
    /// Whether charges are billed monthly for yearly plans.
    pub bill_charges_monthly: Option<bool>,
    /// Number of active subscriptions using this plan.
    pub active_subscriptions_count: Option<i32>,
    /// Number of draft invoices for this plan.
    pub draft_invoices_count: Option<i32>,
    /// Parent plan ID (for plan overrides).
    pub parent_id: Option<Uuid>,
    /// Charges associated with this plan.
    pub charges: Option<Vec<PlanCharge>>,
    /// Taxes associated with this plan.
    pub taxes: Option<Vec<PlanTax>>,
    /// Minimum commitment for this plan.
    pub minimum_commitment: Option<PlanMinimumCommitment>,
    /// Usage thresholds for progressive billing.
    pub usage_thresholds: Option<Vec<UsageThreshold>>,
}

/// Billing interval for a plan.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PlanInterval {
    /// Weekly billing.
    Weekly,
    /// Monthly billing.
    Monthly,
    /// Quarterly billing.
    Quarterly,
    /// Semi-annual billing.
    Semiannual,
    /// Yearly billing.
    Yearly,
}

/// Charge model types.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ChargeModel {
    /// Standard pricing model.
    Standard,
    /// Graduated pricing model.
    Graduated,
    /// Volume pricing model.
    Volume,
    /// Package pricing model.
    Package,
    /// Percentage pricing model.
    Percentage,
    /// Graduated percentage pricing model.
    GraduatedPercentage,
    /// Dynamic pricing model.
    Dynamic,
}

/// Represents a charge in a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanCharge {
    /// Unique identifier for the charge in Lago.
    pub lago_id: Option<Uuid>,
    /// The Lago ID of the billable metric.
    pub lago_billable_metric_id: Option<Uuid>,
    /// The billable metric ID to reference.
    pub billable_metric_id: Option<Uuid>,
    /// When the charge was created.
    pub created_at: Option<DateTime<Utc>>,
    /// The charge model to use.
    pub charge_model: ChargeModel,
    /// Whether the charge is invoiceable.
    pub invoiceable: Option<bool>,
    /// Display name for the charge on invoices.
    pub invoice_display_name: Option<String>,
    /// Whether the charge is billed in advance.
    pub pay_in_advance: Option<bool>,
    /// Whether the charge is prorated.
    pub prorated: Option<bool>,
    /// Minimum amount in cents for this charge.
    pub min_amount_cents: Option<i64>,
    /// Charge properties (model-specific configuration).
    pub properties: Option<serde_json::Value>,
    /// Tax codes for this charge.
    pub tax_codes: Option<Vec<String>>,
    /// Taxes applied to this charge.
    pub taxes: Option<Vec<PlanTax>>,
    /// Filters for differentiated pricing.
    pub filters: Option<Vec<ChargeFilter>>,
    /// The regroup paid fees option.
    pub regroup_paid_fees: Option<String>,
}

/// Represents a charge filter for differentiated pricing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeFilter {
    /// Invoice display name for this filter.
    pub invoice_display_name: Option<String>,
    /// Filter properties.
    pub properties: Option<serde_json::Value>,
    /// Filter values mapping.
    pub values: Option<serde_json::Value>,
}

/// Represents a tax applied to a plan or charge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanTax {
    /// Unique identifier for the tax in Lago.
    pub lago_id: Option<Uuid>,
    /// Name of the tax.
    pub name: Option<String>,
    /// Code for the tax.
    pub code: Option<String>,
    /// Tax rate as a string (percentage).
    pub rate: Option<String>,
    /// Description of the tax.
    pub description: Option<String>,
    /// Whether tax is applied by default.
    pub applied_to_organization: Option<bool>,
}

/// Minimum commitment for a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanMinimumCommitment {
    /// Unique identifier in Lago.
    pub lago_id: Option<Uuid>,
    /// Plan ID this commitment belongs to.
    pub plan_code: Option<String>,
    /// Minimum commitment amount in cents.
    pub amount_cents: Option<i64>,
    /// Invoice display name for the minimum commitment.
    pub invoice_display_name: Option<String>,
    /// When the commitment was created.
    pub created_at: Option<DateTime<Utc>>,
    /// When the commitment was last updated.
    pub updated_at: Option<DateTime<Utc>>,
    /// Tax codes for the minimum commitment.
    pub tax_codes: Option<Vec<String>>,
    /// Taxes applied to the minimum commitment.
    pub taxes: Option<Vec<PlanTax>>,
}
