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
pub struct PlanInput {
    #[serde(rename = "plan")]
    pub plan: Box<crate::models::PlanInputPlan>,
}

impl PlanInput {
    pub fn new(plan: crate::models::PlanInputPlan) -> PlanInput {
        PlanInput {
            plan: Box::new(plan),
        }
    }
}


