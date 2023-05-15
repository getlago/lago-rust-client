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
pub struct InvoiceObjectExtendedAllOf {
    #[serde(rename = "credits")]
    pub credits: Vec<crate::models::CreditObject>,
    #[serde(rename = "fees")]
    pub fees: Vec<crate::models::FeeObject>,
    #[serde(rename = "subscriptions")]
    pub subscriptions: Vec<crate::models::SubscriptionObject>,
}

impl InvoiceObjectExtendedAllOf {
    pub fn new(credits: Vec<crate::models::CreditObject>, fees: Vec<crate::models::FeeObject>, subscriptions: Vec<crate::models::SubscriptionObject>) -> InvoiceObjectExtendedAllOf {
        InvoiceObjectExtendedAllOf {
            credits,
            fees,
            subscriptions,
        }
    }
}


