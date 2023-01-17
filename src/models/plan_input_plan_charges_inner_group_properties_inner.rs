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
pub struct PlanInputPlanChargesInnerGroupPropertiesInner {
    #[serde(rename = "group_id", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<serde_json::Value>,
}

impl PlanInputPlanChargesInnerGroupPropertiesInner {
    pub fn new() -> PlanInputPlanChargesInnerGroupPropertiesInner {
        PlanInputPlanChargesInnerGroupPropertiesInner {
            group_id: None,
            values: None,
        }
    }
}

