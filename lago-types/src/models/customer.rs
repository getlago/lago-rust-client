use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use uuid::Uuid;

/// Represents a customer in the Lago billing system.
///
/// This struct contains all the information about a customer, including
/// their identification, contact details, billing configuration, and
/// associated metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub lago_id: Option<Uuid>,
    pub sequential_id: Option<i32>,
    pub slug: Option<String>,
    pub external_id: Option<String>,
    pub applicable_timezone: String,
    pub created_at: DateTime<Utc>,
    pub billing_entity_code: String,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub currency: Option<String>,
    pub email: Option<String>,
    pub legal_name: Option<String>,
    pub legal_number: Option<String>,
    pub logo_url: Option<String>,
    pub name: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub account_type: CustomerAccountType,
    pub customer_type: Option<CustomerType>,
    pub phone: Option<String>,
    pub state: Option<String>,
    pub tax_identification_number: Option<String>,
    pub timezone: Option<String>,
    pub url: Option<String>,
    pub zipcode: Option<String>,
    pub net_payment_term: Option<i32>,
    pub updated_at: DateTime<Utc>,
    pub finalize_zero_amount_invoice: CustomerFinalizeZeroAmountInvoice,
    pub skip_invoice_custom_sections: bool,
    pub billing_configuration: CustomerBillingConfiguration,
    pub shipping_address: CustomerShippingAddress,
    pub metadata: Vec<CustomerMetadata>,
    pub integration_customers: Vec<CustomerIntegration>,
}

/// Defines the type of customer account.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CustomerAccountType {
    Customer,
    Partner,
}

/// Defines the type of customer entity.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CustomerType {
    Company,
    Individual,
}

/// Configuration for handling zero-amount invoices.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CustomerFinalizeZeroAmountInvoice {
    Inherit,
    Finalize,
    Skip,
}

/// Billing configuration settings for a customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerBillingConfiguration {
    pub invoice_grace_period: Option<i32>,
    pub payment_provider: Option<CustomerPaymentProvider>,
    pub payment_provider_code: Option<String>,
    pub provider_customer_id: Option<String>,
    pub sync: Option<bool>,
    pub sync_with_provider: Option<bool>,
    pub document_locale: Option<String>,
    pub provider_payment_methods: Option<Vec<String>>,
}

/// Supported payment providers.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CustomerPaymentProvider {
    Stripe,
    Adyen,
    Gocardless,
    Cashfree,
    Flutterwave,
    Moneyhash,
    /// A provider added to the API after this version of the client, kept as sent.
    #[serde(untagged)]
    #[strum(disabled)]
    Unknown(String),
}

/// Shipping address information for a customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerShippingAddress {
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub zipcode: Option<String>,
}

/// Custom metadata associated with a customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerMetadata {
    pub lago_id: Uuid,
    pub key: String,
    pub value: String,
    pub display_in_invoice: bool,
    pub created_at: DateTime<Utc>,
}

/// Integration configuration for connecting with third-party systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerIntegration {
    pub lago_id: Uuid,
    #[serde(alias = "type")]
    pub integration_type: CustomerIntegrationType,
    pub integration_code: Option<String>,
    pub external_customer_id: Option<String>,
    /// Absent when it was never set on the integration.
    #[serde(default)]
    pub sync_with_provider: bool,
    pub subsidiary_id: Option<String>,
}

/// Supported integration types.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CustomerIntegrationType {
    Netsuite,
    Anrok,
    Avalara,
    Xero,
    Hubspot,
    Salesforce,
    /// A type added to the API after this version of the client, kept as sent.
    #[serde(untagged)]
    #[strum(disabled)]
    Unknown(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn integration_json(integration_type: &str, sync_with_provider: &str) -> String {
        format!(
            r#"{{
                "lago_id": "1a901a90-1a90-1a90-1a90-1a901a901a90",
                "external_customer_id": "crm_123",
                "type": "{integration_type}",
                "integration_code": "my_integration",
                {sync_with_provider}
                "subsidiary_id": null
            }}"#
        )
    }

    #[test]
    fn deserializes_every_integration_type_of_the_api() {
        for integration_type in [
            "netsuite",
            "anrok",
            "avalara",
            "xero",
            "hubspot",
            "salesforce",
        ] {
            let json = integration_json(integration_type, r#""sync_with_provider": true,"#);
            let integration: CustomerIntegration = serde_json::from_str(&json).unwrap();
            assert_eq!(
                serde_json::to_value(&integration.integration_type).unwrap(),
                integration_type
            );
        }
    }

    #[test]
    fn deserializes_an_unknown_integration_type() {
        let json = integration_json("quickbooks", r#""sync_with_provider": true,"#);
        let integration: CustomerIntegration = serde_json::from_str(&json).unwrap();
        assert!(matches!(
            &integration.integration_type,
            CustomerIntegrationType::Unknown(value) if value == "quickbooks"
        ));
        assert_eq!(
            serde_json::to_value(&integration.integration_type).unwrap(),
            "quickbooks"
        );
    }

    #[test]
    fn does_not_parse_unknown_as_an_integration_type() {
        assert!(CustomerIntegrationType::from_str("unknown").is_err());
    }

    #[test]
    fn deserializes_every_payment_provider_of_the_api() {
        for payment_provider in [
            "stripe",
            "gocardless",
            "cashfree",
            "adyen",
            "flutterwave",
            "moneyhash",
        ] {
            let json = format!(r#"{{"payment_provider": "{payment_provider}"}}"#);
            let billing_configuration: CustomerBillingConfiguration =
                serde_json::from_str(&json).unwrap();
            assert_eq!(
                serde_json::to_value(&billing_configuration.payment_provider).unwrap(),
                payment_provider
            );
        }
    }

    #[test]
    fn deserializes_an_unknown_payment_provider() {
        let billing_configuration: CustomerBillingConfiguration =
            serde_json::from_str(r#"{"payment_provider": "stripe_v2"}"#).unwrap();
        assert!(matches!(
            &billing_configuration.payment_provider,
            Some(CustomerPaymentProvider::Unknown(value)) if value == "stripe_v2"
        ));
        assert_eq!(
            serde_json::to_value(&billing_configuration.payment_provider).unwrap(),
            "stripe_v2"
        );
    }

    #[test]
    fn does_not_parse_unknown_as_a_payment_provider() {
        assert!(CustomerPaymentProvider::from_str("unknown").is_err());
    }

    #[test]
    fn defaults_a_missing_sync_with_provider_to_false() {
        let integration: CustomerIntegration =
            serde_json::from_str(&integration_json("anrok", "")).unwrap();
        assert!(!integration.sync_with_provider);
    }

    #[test]
    fn keeps_an_explicit_sync_with_provider() {
        let integration: CustomerIntegration =
            serde_json::from_str(&integration_json("anrok", r#""sync_with_provider": true,"#))
                .unwrap();
        assert!(integration.sync_with_provider);
    }

    #[test]
    fn rejects_an_integration_type_that_is_not_a_string() {
        let json = integration_json("anrok", "").replace(r#""anrok""#, "42");
        assert!(serde_json::from_str::<CustomerIntegration>(&json).is_err());
    }

    #[test]
    fn deserializes_a_null_payment_provider_as_none() {
        let billing_configuration: CustomerBillingConfiguration =
            serde_json::from_str(r#"{"payment_provider": null}"#).unwrap();
        assert!(billing_configuration.payment_provider.is_none());
    }

    #[test]
    fn parses_the_new_variants_from_strings() {
        for (value, expected) in [
            ("avalara", CustomerIntegrationType::Avalara),
            ("xero", CustomerIntegrationType::Xero),
            ("hubspot", CustomerIntegrationType::Hubspot),
            ("salesforce", CustomerIntegrationType::Salesforce),
        ] {
            assert_eq!(
                serde_json::to_value(CustomerIntegrationType::from_str(value).unwrap()).unwrap(),
                serde_json::to_value(expected).unwrap()
            );
        }
        for (value, expected) in [
            ("cashfree", CustomerPaymentProvider::Cashfree),
            ("flutterwave", CustomerPaymentProvider::Flutterwave),
            ("moneyhash", CustomerPaymentProvider::Moneyhash),
        ] {
            assert_eq!(
                serde_json::to_value(CustomerPaymentProvider::from_str(value).unwrap()).unwrap(),
                serde_json::to_value(expected).unwrap()
            );
        }
    }

    /// The Lago MCP server reads a customer, re-serializes it with `serde_json::json!`
    /// and may read it back: every value the API returns must survive that round trip.
    #[test]
    fn round_trips_a_customer_with_new_and_unknown_providers() {
        let json = r#"{
            "lago_id": "2b012b01-2b01-2b01-2b01-2b012b012b01",
            "external_id": "customer_123",
            "applicable_timezone": "UTC",
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-01T00:00:00Z",
            "billing_entity_code": "default",
            "account_type": "customer",
            "finalize_zero_amount_invoice": "inherit",
            "skip_invoice_custom_sections": false,
            "billing_configuration": {
                "payment_provider": "stripe_v2",
                "payment_provider_code": "my_provider"
            },
            "shipping_address": {},
            "metadata": [],
            "integration_customers": [
                {
                    "lago_id": "3c123c12-3c12-3c12-3c12-3c123c123c12",
                    "type": "salesforce",
                    "integration_code": "crm",
                    "external_customer_id": null,
                    "subsidiary_id": null
                },
                {
                    "lago_id": "4d234d23-4d23-4d23-4d23-4d234d234d23",
                    "type": "quickbooks",
                    "integration_code": "accounting",
                    "external_customer_id": "qb_1",
                    "sync_with_provider": true,
                    "subsidiary_id": null
                }
            ]
        }"#;
        let customer: Customer = serde_json::from_str(json).unwrap();

        let value = serde_json::json!({ "customer": customer });
        let serialized = &value["customer"];
        assert_eq!(
            serialized["billing_configuration"]["payment_provider"],
            "stripe_v2"
        );
        assert_eq!(
            serialized["integration_customers"][0]["integration_type"],
            "salesforce"
        );
        assert_eq!(
            serialized["integration_customers"][1]["integration_type"],
            "quickbooks"
        );

        let read_back: Customer = serde_json::from_value(serialized.clone()).unwrap();
        assert_eq!(serde_json::to_value(&read_back).unwrap(), *serialized);
        assert!(!read_back.integration_customers[0].sync_with_provider);
        assert!(read_back.integration_customers[1].sync_with_provider);
    }
}
