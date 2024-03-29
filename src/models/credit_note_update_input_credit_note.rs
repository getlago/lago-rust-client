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
pub struct CreditNoteUpdateInputCreditNote {
    /// Refund status
    #[serde(rename = "refund_status")]
    pub refund_status: RefundStatus,
}

impl CreditNoteUpdateInputCreditNote {
    pub fn new(refund_status: RefundStatus) -> CreditNoteUpdateInputCreditNote {
        CreditNoteUpdateInputCreditNote {
            refund_status,
        }
    }
}

/// Refund status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RefundStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for RefundStatus {
    fn default() -> RefundStatus {
        Self::Pending
    }
}

