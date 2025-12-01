use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents the usage-based billing data for a customer within the current period.
///
/// This struct contains the aggregated usage information including charges,
/// billing period dates, and monetary amounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerUsage {
    /// Start of the billing period.
    pub from_datetime: DateTime<Utc>,
    /// End of the billing period.
    pub to_datetime: DateTime<Utc>,
    /// Date when the invoice will be issued.
    pub issuing_date: NaiveDate,
    /// Lago invoice ID if already generated.
    pub lago_invoice_id: Option<Uuid>,
    /// Currency code (ISO 4217).
    pub currency: String,
    /// Total charge amount in cents before taxes.
    pub amount_cents: i64,
    /// Total tax amount in cents.
    pub taxes_amount_cents: i64,
    /// Grand total amount in cents (charges + taxes).
    pub total_amount_cents: i64,
    /// Detailed usage breakdown by charge.
    pub charges_usage: Vec<ChargeUsage>,
}

/// Represents the usage details for a specific charge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeUsage {
    /// Aggregated quantity/units consumed.
    pub units: String,
    /// Total aggregated units consumed.
    #[serde(default)]
    pub total_aggregated_units: Option<String>,
    /// Number of events processed.
    pub events_count: i32,
    /// Charge cost in cents.
    pub amount_cents: i64,
    /// Currency code for the amount.
    pub amount_currency: String,
    /// The charge configuration.
    pub charge: ChargeUsageCharge,
    /// The billable metric this charge is based on.
    pub billable_metric: ChargeUsageBillableMetric,
    /// Filter-based usage breakdowns.
    #[serde(default)]
    pub filters: Vec<ChargeUsageFilter>,
    /// Grouped usage consumption data.
    #[serde(default)]
    pub grouped_usage: Vec<GroupedUsage>,
}

/// Represents the charge configuration in usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeUsageCharge {
    /// Lago ID of the charge.
    pub lago_id: Uuid,
    /// The charge model type.
    pub charge_model: String,
    /// Display name for the invoice.
    pub invoice_display_name: Option<String>,
}

/// Represents the billable metric in usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeUsageBillableMetric {
    /// Lago ID of the billable metric.
    pub lago_id: Uuid,
    /// Name of the billable metric.
    pub name: String,
    /// Code of the billable metric.
    pub code: String,
    /// Aggregation type.
    pub aggregation_type: String,
}

/// Represents filter-based usage breakdown.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeUsageFilter {
    /// Lago ID of the filter.
    pub lago_id: Option<Uuid>,
    /// Filter key.
    pub key: Option<String>,
    /// Filter value.
    pub value: Option<String>,
    /// Units consumed with this filter.
    pub units: String,
    /// Amount in cents for this filter.
    pub amount_cents: i64,
    /// Number of events with this filter.
    pub events_count: i32,
    /// Invoice display name for the filter.
    pub invoice_display_name: Option<String>,
}

/// Represents grouped usage data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupedUsage {
    /// Amount in cents for this group.
    pub amount_cents: i64,
    /// Number of events in this group.
    pub events_count: i32,
    /// Units consumed in this group.
    pub units: String,
    /// Grouped by key-value pairs.
    #[serde(default)]
    pub grouped_by: std::collections::HashMap<String, Option<String>>,
    /// Filters applied to this group.
    #[serde(default)]
    pub filters: Vec<ChargeUsageFilter>,
}
