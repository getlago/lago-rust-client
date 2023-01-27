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
pub struct WalletUpdateInputWallet {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "expiration_at", skip_serializing_if = "Option::is_none")]
    pub expiration_at: Option<String>,
}

impl WalletUpdateInputWallet {
    pub fn new() -> WalletUpdateInputWallet {
        WalletUpdateInputWallet {
            name: None,
            expiration_at: None,
        }
    }
}


