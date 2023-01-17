/*
 * Lago API documentation
 *
 * Lago API allows your application to push customer information and metrics (events) from your application to the billing application.
 *
 * The version of the OpenAPI document: 0.20.0-beta
 * Contact: tech@getlago.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BillableMetricInput {
    #[serde(rename = "billable_metric", skip_serializing_if = "Option::is_none")]
    pub billable_metric: Option<Box<crate::models::BillableMetricInputBillableMetric>>,
}

impl BillableMetricInput {
    pub fn new() -> BillableMetricInput {
        BillableMetricInput {
            billable_metric: None,
        }
    }
}

