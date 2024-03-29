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
pub struct GetCustomerPortalUrl200Response {
    #[serde(rename = "customer")]
    pub customer: Box<crate::models::GetCustomerPortalUrl200ResponseCustomer>,
}

impl GetCustomerPortalUrl200Response {
    pub fn new(customer: crate::models::GetCustomerPortalUrl200ResponseCustomer) -> GetCustomerPortalUrl200Response {
        GetCustomerPortalUrl200Response {
            customer: Box::new(customer),
        }
    }
}


