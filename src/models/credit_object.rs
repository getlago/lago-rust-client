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
pub struct CreditObject {
    #[serde(rename = "lago_id", skip_serializing_if = "Option::is_none")]
    pub lago_id: Option<String>,
    #[serde(rename = "amount_cents", skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i32>,
    #[serde(rename = "amount_currency", skip_serializing_if = "Option::is_none")]
    pub amount_currency: Option<String>,
    #[serde(rename = "item", skip_serializing_if = "Option::is_none")]
    pub item: Option<Box<crate::models::CreditObjectItem>>,
}

impl CreditObject {
    pub fn new() -> CreditObject {
        CreditObject {
            lago_id: None,
            amount_cents: None,
            amount_currency: None,
            item: None,
        }
    }
}

