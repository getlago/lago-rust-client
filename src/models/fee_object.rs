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
pub struct FeeObject {
    #[serde(rename = "lago_id", skip_serializing_if = "Option::is_none")]
    pub lago_id: Option<String>,
    #[serde(rename = "lago_group_id", skip_serializing_if = "Option::is_none")]
    pub lago_group_id: Option<String>,
    #[serde(rename = "amount_cents", skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i32>,
    #[serde(rename = "amount_currency", skip_serializing_if = "Option::is_none")]
    pub amount_currency: Option<String>,
    #[serde(rename = "vat_amount_cents", skip_serializing_if = "Option::is_none")]
    pub vat_amount_cents: Option<i32>,
    #[serde(rename = "vat_amount_currency", skip_serializing_if = "Option::is_none")]
    pub vat_amount_currency: Option<String>,
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<f32>,
    #[serde(rename = "events_count", skip_serializing_if = "Option::is_none")]
    pub events_count: Option<i32>,
    #[serde(rename = "item", skip_serializing_if = "Option::is_none")]
    pub item: Option<Box<crate::models::FeeObjectItem>>,
}

impl FeeObject {
    pub fn new() -> FeeObject {
        FeeObject {
            lago_id: None,
            lago_group_id: None,
            amount_cents: None,
            amount_currency: None,
            vat_amount_cents: None,
            vat_amount_currency: None,
            units: None,
            events_count: None,
            item: None,
        }
    }
}


