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
pub struct AddOnInputAddOn {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "amount_cents", skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i32>,
    #[serde(rename = "amount_currency", skip_serializing_if = "Option::is_none")]
    pub amount_currency: Option<String>,
}

impl AddOnInputAddOn {
    pub fn new() -> AddOnInputAddOn {
        AddOnInputAddOn {
            name: None,
            code: None,
            description: None,
            amount_cents: None,
            amount_currency: None,
        }
    }
}


