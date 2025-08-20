use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use uuid::Uuid;

/// Represents a billable metric in the Lago billing system.
///
/// A billable metric defines how usage events are aggregated for billing purposes.
/// It specifies the aggregation method, field to aggregate on, and filters for
/// differentiated pricing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillableMetric {
    pub lago_id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub aggregation_type: BillableMetricAggregationType,
    pub recurring: bool,
    pub rounding_function: Option<BillableMetricRoundingFunction>,
    pub rounding_precision: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub expression: Option<String>,
    pub field_name: Option<String>,
    pub weighted_interval: Option<BillableMetricWeightedInterval>,
    pub filters: Vec<BillableMetricFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum BillableMetricAggregationType {
    CountAgg,
    SumAgg,
    MaxAgg,
    UniqueCountAgg,
    WeightedSumAgg,
    LatestAgg,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum BillableMetricRoundingFunction {
    Ceil,
    Floor,
    Round,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum BillableMetricWeightedInterval {
    Seconds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillableMetricFilter {
    pub key: String,
    pub values: Vec<String>,
}

impl BillableMetricFilter {
    /// Creates a new billable metric filter.
    ///
    /// # Arguments
    /// * `key` - The filter key to add to the event properties payload
    /// * `values` - List of possible filter values
    ///
    /// # Returns
    /// A new `BillableMetricFilter` instance
    pub fn new(key: String, values: Vec<String>) -> Self {
        Self { key, values }
    }
}
