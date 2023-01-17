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
pub struct CreditNotes {
    #[serde(rename = "credit_notes", skip_serializing_if = "Option::is_none")]
    pub credit_notes: Option<Vec<crate::models::CreditNoteObject>>,
}

impl CreditNotes {
    pub fn new() -> CreditNotes {
        CreditNotes {
            credit_notes: None,
        }
    }
}


