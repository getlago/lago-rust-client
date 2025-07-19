use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a customer in the Lago billing system.
///
/// This struct contains all the information about a customer, including
/// their identification, contact details, billing configuration, and
/// associated metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub lago_id: Uuid,
    pub sequential_id: i32,
    pub slug: String,
    pub external_id: String,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerAccountType {
    Customer,
    Partner,
}

/// Defines the type of customer entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerType {
    Company,
    Individual,
}

/// Configuration for handling zero-amount invoices.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerPaymentProvider {
    Stripe,
    Adyen,
    Gocardless,
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
    pub integration_code: String,
    pub external_customer_id: String,
    pub sync_with_provider: bool,
    pub subsidiary_id: String,
}

/// Supported integration types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustomerIntegrationType {
    Netsuite,
    Anrok,
}
