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
pub struct WalletTransactionInputWalletTransaction {
    #[serde(rename = "wallet_id", skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
    #[serde(rename = "paid_credits", skip_serializing_if = "Option::is_none")]
    pub paid_credits: Option<f32>,
    #[serde(rename = "granted_credits", skip_serializing_if = "Option::is_none")]
    pub granted_credits: Option<f32>,
}

impl WalletTransactionInputWalletTransaction {
    pub fn new() -> WalletTransactionInputWalletTransaction {
        WalletTransactionInputWalletTransaction {
            wallet_id: None,
            paid_credits: None,
            granted_credits: None,
        }
    }
}


