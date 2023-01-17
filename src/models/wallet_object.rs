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
pub struct WalletObject {
    #[serde(rename = "lago_id", skip_serializing_if = "Option::is_none")]
    pub lago_id: Option<String>,
    #[serde(rename = "lago_customer_id", skip_serializing_if = "Option::is_none")]
    pub lago_customer_id: Option<String>,
    #[serde(rename = "external_customer_id", skip_serializing_if = "Option::is_none")]
    pub external_customer_id: Option<String>,
    /// Status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "rate_amount", skip_serializing_if = "Option::is_none")]
    pub rate_amount: Option<f32>,
    #[serde(rename = "credits_balance", skip_serializing_if = "Option::is_none")]
    pub credits_balance: Option<f32>,
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<f32>,
    #[serde(rename = "consumed_credits", skip_serializing_if = "Option::is_none")]
    pub consumed_credits: Option<f32>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "expiration_at", skip_serializing_if = "Option::is_none")]
    pub expiration_at: Option<String>,
    #[serde(rename = "last_balance_sync_at", skip_serializing_if = "Option::is_none")]
    pub last_balance_sync_at: Option<String>,
    #[serde(rename = "last_consumed_credit_at", skip_serializing_if = "Option::is_none")]
    pub last_consumed_credit_at: Option<String>,
    #[serde(rename = "terminated_at", skip_serializing_if = "Option::is_none")]
    pub terminated_at: Option<String>,
}

impl WalletObject {
    pub fn new() -> WalletObject {
        WalletObject {
            lago_id: None,
            lago_customer_id: None,
            external_customer_id: None,
            status: None,
            currency: None,
            name: None,
            rate_amount: None,
            credits_balance: None,
            balance: None,
            consumed_credits: None,
            created_at: None,
            expiration_at: None,
            last_balance_sync_at: None,
            last_consumed_credit_at: None,
            terminated_at: None,
        }
    }
}

/// Status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "terminated")]
    Terminated,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}

