use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents the current usage data for a customer's subscription.
///
/// This struct contains information about usage-based billing data
/// within the current billing period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerUsage {
    /// Start of the billing period
    pub from_datetime: DateTime<Utc>,
    /// End of the billing period
    pub to_datetime: DateTime<Utc>,
    /// Invoice issuance date
    pub issuing_date: String,
    /// Associated invoice identifier (if exists)
    pub lago_invoice_id: Option<Uuid>,
    /// Currency code (e.g., USD, EUR)
    pub currency: String,
    /// Total charges amount in cents (excluding taxes)
    pub amount_cents: i64,
    /// Tax amount in cents
    pub taxes_amount_cents: i64,
    /// Grand total in cents (amount + taxes)
    pub total_amount_cents: i64,
    /// Array of charge usage line items
    pub charges_usage: Vec<ChargeUsage>,
}

/// Represents usage data for a specific charge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeUsage {
    /// Aggregated unit quantity
    pub units: String,
    /// Total units aggregated across the period
    pub total_aggregated_units: Option<String>,
    /// Number of events processed
    pub events_count: i64,
    /// Charge amount in cents
    pub amount_cents: i64,
    /// Currency code
    pub amount_currency: String,
    /// Custom pricing unit details
    pub pricing_unit_details: Option<PricingUnitDetails>,
    /// Charge configuration
    pub charge: ChargeInfo,
    /// Billable metric details
    pub billable_metric: BillableMetricInfo,
    /// Applied filter breakdowns
    pub filters: Vec<ChargeFilterUsage>,
    /// Grouped usage data
    pub grouped_usage: Vec<GroupedUsage>,
}

/// Pricing unit details for custom pricing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingUnitDetails {
    /// Amount in cents
    pub amount_cents: Option<i64>,
    /// Short name for the pricing unit
    pub short_name: Option<String>,
    /// Conversion rate
    pub conversion_rate: Option<String>,
}

/// Basic charge information within usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeInfo {
    /// Unique identifier for the charge in Lago
    pub lago_id: Uuid,
    /// Charge model (standard, graduated, volume, package, percentage)
    pub charge_model: String,
    /// Display name for invoices
    pub invoice_display_name: Option<String>,
}

/// Basic billable metric information within usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillableMetricInfo {
    /// Unique identifier for the billable metric in Lago
    pub lago_id: Uuid,
    /// Name of the billable metric
    pub name: String,
    /// Unique code for the billable metric
    pub code: String,
    /// Aggregation type (count_agg, sum_agg, max_agg, etc.)
    pub aggregation_type: String,
}

/// Filter usage breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeFilterUsage {
    /// Units for this filter
    pub units: String,
    /// Total aggregated units for this filter
    pub total_aggregated_units: Option<String>,
    /// Amount in cents for this filter
    pub amount_cents: i64,
    /// Number of events for this filter
    pub events_count: i64,
    /// Display name for invoices
    pub invoice_display_name: Option<String>,
    /// Filter values
    pub values: serde_json::Value,
}

/// Grouped usage data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupedUsage {
    /// Units for this group
    pub units: String,
    /// Total aggregated units for this group
    pub total_aggregated_units: Option<String>,
    /// Amount in cents for this group
    pub amount_cents: i64,
    /// Number of events for this group
    pub events_count: i64,
    /// Grouped by values
    pub grouped_by: serde_json::Value,
    /// Filters within this group
    pub filters: Vec<ChargeFilterUsage>,
}
