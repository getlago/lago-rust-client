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
pub struct CustomerInputCustomer {
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "address_line1", skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    #[serde(rename = "address_line2", skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "zipode", skip_serializing_if = "Option::is_none")]
    pub zipode: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "lago_url", skip_serializing_if = "Option::is_none")]
    pub lago_url: Option<String>,
    #[serde(rename = "legal_name", skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(rename = "legal_number", skip_serializing_if = "Option::is_none")]
    pub legal_number: Option<String>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "billing_configuration", skip_serializing_if = "Option::is_none")]
    pub billing_configuration: Option<crate::models::BillingConfigurationCustomer>,
}

impl CustomerInputCustomer {
    pub fn new() -> CustomerInputCustomer {
        CustomerInputCustomer {
            external_id: None,
            name: None,
            country: None,
            address_line1: None,
            address_line2: None,
            state: None,
            zipode: None,
            email: None,
            city: None,
            url: None,
            phone: None,
            lago_url: None,
            legal_name: None,
            legal_number: None,
            currency: None,
            timezone: None,
            billing_configuration: None,
        }
    }
}


