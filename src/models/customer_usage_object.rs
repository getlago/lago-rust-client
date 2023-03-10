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
pub struct CustomerUsageObject {
    #[serde(rename = "from_datetime", skip_serializing_if = "Option::is_none")]
    pub from_datetime: Option<String>,
    #[serde(rename = "to_datetime", skip_serializing_if = "Option::is_none")]
    pub to_datetime: Option<String>,
    #[serde(rename = "issuing_date", skip_serializing_if = "Option::is_none")]
    pub issuing_date: Option<String>,
    #[serde(rename = "amount_cents", skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i32>,
    #[serde(rename = "amount_currency", skip_serializing_if = "Option::is_none")]
    pub amount_currency: Option<String>,
    #[serde(rename = "total_amount_cents", skip_serializing_if = "Option::is_none")]
    pub total_amount_cents: Option<i32>,
    #[serde(rename = "total_amount_currency", skip_serializing_if = "Option::is_none")]
    pub total_amount_currency: Option<String>,
    #[serde(rename = "vat_amount_cents", skip_serializing_if = "Option::is_none")]
    pub vat_amount_cents: Option<i32>,
    #[serde(rename = "vat_amount_currency", skip_serializing_if = "Option::is_none")]
    pub vat_amount_currency: Option<String>,
    #[serde(rename = "charges_usage", skip_serializing_if = "Option::is_none")]
    pub charges_usage: Option<Vec<crate::models::ChargeUsageObject>>,
}

impl CustomerUsageObject {
    pub fn new() -> CustomerUsageObject {
        CustomerUsageObject {
            from_datetime: None,
            to_datetime: None,
            issuing_date: None,
            amount_cents: None,
            amount_currency: None,
            total_amount_cents: None,
            total_amount_currency: None,
            vat_amount_cents: None,
            vat_amount_currency: None,
            charges_usage: None,
        }
    }
}


