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
pub struct WalletTransactions {
    #[serde(rename = "wallet_transactions")]
    pub wallet_transactions: Vec<crate::models::WalletTransactionObject>,
}

impl WalletTransactions {
    pub fn new(wallet_transactions: Vec<crate::models::WalletTransactionObject>) -> WalletTransactions {
        WalletTransactions {
            wallet_transactions,
        }
    }
}

