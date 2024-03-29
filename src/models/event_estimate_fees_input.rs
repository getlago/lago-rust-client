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
pub struct EventEstimateFeesInput {
    #[serde(rename = "event")]
    pub event: Box<crate::models::EventEstimateFeesInputEvent>,
}

impl EventEstimateFeesInput {
    pub fn new(event: crate::models::EventEstimateFeesInputEvent) -> EventEstimateFeesInput {
        EventEstimateFeesInput {
            event: Box::new(event),
        }
    }
}


