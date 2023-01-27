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
pub struct CustomerInput {
    #[serde(rename = "customer", skip_serializing_if = "Option::is_none")]
    pub customer: Option<Box<crate::models::CustomerInputCustomer>>,
}

impl CustomerInput {
    pub fn new() -> CustomerInput {
        CustomerInput {
            customer: None,
        }
    }
}


