/*
 * Lago API documentation
 *
 * Lago API allows your application to push customer information and metrics (events) from your application to the billing application.
 *
 * The version of the OpenAPI document: 0.21.0-beta
 * Contact: tech@getlago.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "subscription", skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Box<crate::models::SubscriptionObject>>,
}

impl Subscription {
    pub fn new() -> Subscription {
        Subscription {
            subscription: None,
        }
    }
}


