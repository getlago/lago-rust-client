/*
 * Lago API documentation
 *
 * Lago API allows your application to push customer information and metrics (events) from your application to the billing application.
 *
 * The version of the OpenAPI document: 0.32.0-beta
 * Contact: tech@getlago.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChargeUsageObjectBillableMetric {
    #[serde(rename = "lago_id", skip_serializing_if = "Option::is_none")]
    pub lago_id: Option<uuid::Uuid>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Aggregation type
    #[serde(rename = "aggregation_type", skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<AggregationType>,
}

impl ChargeUsageObjectBillableMetric {
    pub fn new() -> ChargeUsageObjectBillableMetric {
        ChargeUsageObjectBillableMetric {
            lago_id: None,
            name: None,
            code: None,
            aggregation_type: None,
        }
    }
}

/// Aggregation type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AggregationType {
    #[serde(rename = "count_agg")]
    CountAgg,
    #[serde(rename = "sum_agg")]
    SumAgg,
    #[serde(rename = "max_agg")]
    MaxAgg,
    #[serde(rename = "unique_count_agg")]
    UniqueCountAgg,
    #[serde(rename = "recurring_count_agg")]
    RecurringCountAgg,
}

impl Default for AggregationType {
    fn default() -> AggregationType {
        Self::CountAgg
    }
}

