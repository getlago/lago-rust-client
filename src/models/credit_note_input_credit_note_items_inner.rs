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
pub struct CreditNoteInputCreditNoteItemsInner {
    #[serde(rename = "fee_id", skip_serializing_if = "Option::is_none")]
    pub fee_id: Option<String>,
    #[serde(rename = "amount_cents", skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i32>,
}

impl CreditNoteInputCreditNoteItemsInner {
    pub fn new() -> CreditNoteInputCreditNoteItemsInner {
        CreditNoteInputCreditNoteItemsInner {
            fee_id: None,
            amount_cents: None,
        }
    }
}


