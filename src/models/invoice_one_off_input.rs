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
pub struct InvoiceOneOffInput {
    #[serde(rename = "invoice")]
    pub invoice: Box<crate::models::InvoiceOneOffInputInvoice>,
}

impl InvoiceOneOffInput {
    pub fn new(invoice: crate::models::InvoiceOneOffInputInvoice) -> InvoiceOneOffInput {
        InvoiceOneOffInput {
            invoice: Box::new(invoice),
        }
    }
}


