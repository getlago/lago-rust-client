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
pub struct WalletTransactionObject {
    #[serde(rename = "lago_id", skip_serializing_if = "Option::is_none")]
    pub lago_id: Option<String>,
    #[serde(rename = "lago_wallet_id", skip_serializing_if = "Option::is_none")]
    pub lago_wallet_id: Option<String>,
    /// Status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Transaction type
    #[serde(rename = "transaction_type", skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<TransactionType>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    #[serde(rename = "credit_amount", skip_serializing_if = "Option::is_none")]
    pub credit_amount: Option<f32>,
    #[serde(rename = "settled_at", skip_serializing_if = "Option::is_none")]
    pub settled_at: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl WalletTransactionObject {
    pub fn new() -> WalletTransactionObject {
        WalletTransactionObject {
            lago_id: None,
            lago_wallet_id: None,
            status: None,
            transaction_type: None,
            amount: None,
            credit_amount: None,
            settled_at: None,
            created_at: None,
        }
    }
}

/// Status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "settled")]
    Settled,
}

impl Default for Status {
    fn default() -> Status {
        Self::Pending
    }
}
/// Transaction type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionType {
    #[serde(rename = "inbound")]
    Inbound,
    #[serde(rename = "outbound")]
    Outbound,
}

impl Default for TransactionType {
    fn default() -> TransactionType {
        Self::Inbound
    }
}

