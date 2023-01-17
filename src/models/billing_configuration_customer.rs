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
pub struct BillingConfigurationCustomer {
    #[serde(rename = "invoice_grace_period", skip_serializing_if = "Option::is_none")]
    pub invoice_grace_period: Option<i32>,
    /// Payment provider type
    #[serde(rename = "payment_provider", skip_serializing_if = "Option::is_none")]
    pub payment_provider: Option<PaymentProvider>,
    #[serde(rename = "provider_customer_id", skip_serializing_if = "Option::is_none")]
    pub provider_customer_id: Option<String>,
    #[serde(rename = "sync_with_provider", skip_serializing_if = "Option::is_none")]
    pub sync_with_provider: Option<bool>,
    #[serde(rename = "vat_rate", skip_serializing_if = "Option::is_none")]
    pub vat_rate: Option<f32>,
}

impl BillingConfigurationCustomer {
    pub fn new() -> BillingConfigurationCustomer {
        BillingConfigurationCustomer {
            invoice_grace_period: None,
            payment_provider: None,
            provider_customer_id: None,
            sync_with_provider: None,
            vat_rate: None,
        }
    }
}

/// Payment provider type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentProvider {
    #[serde(rename = "stripe")]
    Stripe,
    #[serde(rename = "gocardless")]
    Gocardless,
}

impl Default for PaymentProvider {
    fn default() -> PaymentProvider {
        Self::Stripe
    }
}

