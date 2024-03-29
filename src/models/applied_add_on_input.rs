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
pub struct AppliedAddOnInput {
    #[serde(rename = "applied_add_on")]
    pub applied_add_on: Box<crate::models::AppliedAddOnInputAppliedAddOn>,
}

impl AppliedAddOnInput {
    pub fn new(applied_add_on: crate::models::AppliedAddOnInputAppliedAddOn) -> AppliedAddOnInput {
        AppliedAddOnInput {
            applied_add_on: Box::new(applied_add_on),
        }
    }
}


