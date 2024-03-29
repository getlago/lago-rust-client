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
pub struct EventObject {
    #[serde(rename = "lago_id")]
    pub lago_id: uuid::Uuid,
    #[serde(rename = "transaction_id")]
    pub transaction_id: String,
    #[serde(rename = "lago_customer_id")]
    pub lago_customer_id: uuid::Uuid,
    #[serde(rename = "external_customer_id")]
    pub external_customer_id: String,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(rename = "lago_subscription_id")]
    pub lago_subscription_id: uuid::Uuid,
    #[serde(rename = "external_subscription_id")]
    pub external_subscription_id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl EventObject {
    pub fn new(lago_id: uuid::Uuid, transaction_id: String, lago_customer_id: uuid::Uuid, external_customer_id: String, code: String, timestamp: String, lago_subscription_id: uuid::Uuid, external_subscription_id: String, created_at: String) -> EventObject {
        EventObject {
            lago_id,
            transaction_id,
            lago_customer_id,
            external_customer_id,
            code,
            timestamp,
            properties: None,
            lago_subscription_id,
            external_subscription_id,
            created_at,
        }
    }
}


