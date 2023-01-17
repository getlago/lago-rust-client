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
pub struct CreditNoteUpdateInput {
    #[serde(rename = "credit_note", skip_serializing_if = "Option::is_none")]
    pub credit_note: Option<Box<crate::models::CreditNoteUpdateInputCreditNote>>,
}

impl CreditNoteUpdateInput {
    pub fn new() -> CreditNoteUpdateInput {
        CreditNoteUpdateInput {
            credit_note: None,
        }
    }
}


