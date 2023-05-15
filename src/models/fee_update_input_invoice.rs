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
pub struct FeeUpdateInputInvoice {
    /// Status
    #[serde(rename = "payment_status")]
    pub payment_status: PaymentStatus,
}

impl FeeUpdateInputInvoice {
    pub fn new(payment_status: PaymentStatus) -> FeeUpdateInputInvoice {
        FeeUpdateInputInvoice {
            payment_status,
        }
    }
}

/// Status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "refunded")]
    Refunded,
}

impl Default for PaymentStatus {
    fn default() -> PaymentStatus {
        Self::Pending
    }
}
