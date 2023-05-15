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
pub struct CreditNoteInput {
    #[serde(rename = "credit_note")]
    pub credit_note: Box<crate::models::CreditNoteInputCreditNote>,
}

impl CreditNoteInput {
    pub fn new(credit_note: crate::models::CreditNoteInputCreditNote) -> CreditNoteInput {
        CreditNoteInput {
            credit_note: Box::new(credit_note),
        }
    }
}


