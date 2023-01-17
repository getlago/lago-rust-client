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
pub struct EventInputEvent {
    #[serde(rename = "transaction_id", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(rename = "external_customer_id", skip_serializing_if = "Option::is_none")]
    pub external_customer_id: Option<String>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i32>,
    #[serde(rename = "external_subscription_id", skip_serializing_if = "Option::is_none")]
    pub external_subscription_id: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

impl EventInputEvent {
    pub fn new() -> EventInputEvent {
        EventInputEvent {
            transaction_id: None,
            external_customer_id: None,
            code: None,
            timestamp: None,
            external_subscription_id: None,
            properties: None,
        }
    }
}


