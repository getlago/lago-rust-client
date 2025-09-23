use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::EnumString;
use uuid::Uuid;

/// Represents a plan in the Lago billing system.
///
/// This struct contains all the information about a billing plan, including
/// its pricing, charges, taxes, and configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub lago_id: Uuid,
    pub name: String,
    pub code: String,
    pub interval: PlanInterval,
    pub description: Option<String>,
    pub amount_cents: i64,
    pub amount_currency: String,
    pub trial_period: Option<f64>,
    pub pay_in_advance: bool,
    pub invoiceable: bool,
    pub bill_charges_monthly: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub invoice_display_name: Option<String>,
    pub parent_id: Option<Uuid>,
    pub children_count: i32,
    pub active_subscriptions_count: i32,
    pub draft_invoices_count: i32,
    pub minimum_commitment: Option<MinimumCommitment>,
    pub charges: Vec<Charge>,
    pub taxes: Vec<Tax>,
    pub usage_thresholds: Vec<PlanUsageThreshold>,
    pub entitlements: Vec<PlanEntitlement>,
}

/// Defines the billing interval for a plan.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PlanInterval {
    Weekly,
    Monthly,
    Quarterly,
    Semiannual,
    Yearly,
}

/// Minimum commitment configuration for a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumCommitment {
    pub lago_id: Uuid,
    pub amount_cents: i64,
    pub invoice_display_name: Option<String>,
    pub interval: PlanInterval,
    pub created_at: DateTime<Utc>,
    pub taxes: Vec<Tax>,
}

/// Represents a usage-based charge attached to a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Charge {
    pub lago_id: Uuid,
    pub lago_billable_metric_id: Uuid,
    pub billable_metric_code: String,
    pub invoice_display_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub charge_model: ChargeModel,
    pub invoiceable: bool,
    pub pay_in_advance: bool,
    pub prorated: bool,
    pub min_amount_cents: i64,
    pub properties: ChargeProperties,
    pub group_properties: Vec<ChargeGroupProperty>,
    pub billable_metric: PlanBillableMetric,
    pub taxes: Vec<Tax>,
    pub regroup_paid_fees: Option<RegroupPaidFeesType>,
    pub filters: Vec<ChargeFilter>,
}

/// Defines the pricing model used for calculating charges.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ChargeModel {
    Standard,
    Graduated,
    GraduatedPercentage,
    Package,
    Percentage,
    Volume,
    Dynamic,
}

/// Properties that define the pricing structure for a charge.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChargeProperties {
    Standard {
        amount: String,
    },
    Graduated {
        graduated_ranges: Vec<GraduatedRange>,
    },
    GraduatedPercentage {
        graduated_percentage_ranges: Vec<GraduatedPercentageRange>,
    },
    Package {
        amount: String,
        free_units: i64,
        package_size: i64,
    },
    Percentage {
        rate: String,
        fixed_amount: Option<String>,
        free_units_per_events: Option<i32>,
        free_units_per_total_aggregation: Option<String>,
    },
    Volume {
        volume_ranges: Vec<VolumeRange>,
    },
    Dynamic {
        #[serde(flatten)]
        custom_properties: HashMap<String, serde_json::Value>,
    },
}

/// Graduated pricing range for graduated charge model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraduatedRange {
    pub from_value: i64,
    pub to_value: Option<i64>,
    pub flat_amount: String,
    pub per_unit_amount: String,
}

/// Graduated percentage pricing range for graduated percentage charge model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraduatedPercentageRange {
    pub from_value: i64,
    pub to_value: Option<i64>,
    pub rate: String,
    pub flat_amount: String,
}

/// Volume pricing range for volume charge model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeRange {
    pub from_value: i64,
    pub to_value: Option<i64>,
    pub flat_amount: String,
    pub per_unit_amount: String,
}

/// Group-specific properties for charges that can be segmented.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeGroupProperty {
    pub group_id: String,
    pub values: HashMap<String, String>,
    pub invoice_display_name: Option<String>,
}

/// Basic billable metric information associated with a charge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanBillableMetric {
    pub lago_id: Uuid,
    pub name: String,
    pub code: String,
    pub aggregation_type: PlanBillableMetricAggregationType,
    pub created_at: DateTime<Utc>,
    pub field_name: Option<String>,
}

/// Aggregation types supported by billable metrics in plan context.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PlanBillableMetricAggregationType {
    CountAgg,
    SumAgg,
    MaxAgg,
    UniqueCountAgg,
    RecurringCountAgg,
    WeightedSumAgg,
    LatestAgg,
    CustomAgg,
}

/// Defines how paid fees should be regrouped on invoices.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RegroupPaidFeesType {
    Invoice,
    Subscription,
}

/// Filter that can be applied to charges for differentiated pricing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeFilter {
    pub invoice_display_name: Option<String>,
    pub properties: ChargeFilterProperties,
    pub values: HashMap<String, Vec<String>>,
}

/// Properties for charge filters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeFilterProperties {
    #[serde(flatten)]
    pub properties: HashMap<String, serde_json::Value>,
}

/// Tax configuration that can be applied to plans and charges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tax {
    pub lago_id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub rate: f64,
    pub applied_to_organization: bool,
    pub add_ons_count: i32,
    pub charges_count: i32,
    pub commitments_count: i32,
    pub customers_count: i32,
    pub plans_count: i32,
    pub created_at: DateTime<Utc>,
}

/// Usage threshold configuration for plans.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanUsageThreshold {
    pub lago_id: Uuid,
    pub threshold_display_name: Option<String>,
    pub amount_cents: i64,
    pub recurring: bool,
    pub created_at: DateTime<Utc>,
}

/// Feature entitlement associated with a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanEntitlement {
    pub lago_id: Uuid,
    pub lago_feature_id: Uuid,
    pub feature_code: String,
    pub feature_name: String,
    pub created_at: DateTime<Utc>,
    pub privileges: Vec<EntitlementPrivilege>,
}

/// Privilege configuration for a feature entitlement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementPrivilege {
    pub privilege_code: String,
    pub privilege_name: String,
    pub value_type: PrivilegeValueType,
    pub value: serde_json::Value,
    pub plan_value: serde_json::Value,
    pub override_value: Option<serde_json::Value>,
    pub config: HashMap<String, serde_json::Value>,
}

/// Value type for privilege configurations.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PrivilegeValueType {
    Integer,
    Boolean,
    String,
    Select,
}