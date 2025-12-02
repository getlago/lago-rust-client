use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::EnumString;
use uuid::Uuid;

/// Represents a payment in the Lago billing system.
///
/// A payment records a monetary transaction associated with an invoice
/// or payment request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    /// Unique identifier for the payment in Lago.
    pub lago_id: Uuid,
    /// Unique identifier of the customer in Lago.
    pub lago_customer_id: Uuid,
    /// The customer external unique identifier.
    pub external_customer_id: String,
    /// List of invoice IDs associated with the payment.
    pub invoice_ids: Vec<Uuid>,
    /// The unique identifier of the paid resource in Lago.
    pub lago_payable_id: Uuid,
    /// The type of the paid resource (Invoice or PaymentRequest).
    pub payable_type: PayableType,
    /// The amount of the payment in cents.
    pub amount_cents: i64,
    /// The currency of the payment amount.
    pub amount_currency: String,
    /// The status of the payment within the payment provider.
    pub status: String,
    /// The normalized payment status by Lago.
    pub payment_status: PaymentStatus,
    /// The type of payment (manual or provider).
    #[serde(rename = "type")]
    pub payment_type: PaymentType,
    /// Reference for the payment.
    pub reference: Option<String>,
    /// Code of the payment provider.
    pub payment_provider_code: Option<String>,
    /// The type of payment provider.
    pub payment_provider_type: Option<PaymentProviderType>,
    /// DEPRECATED: use provider_payment_id.
    pub external_payment_id: Option<String>,
    /// Unique identifier of the payment within the payment provider.
    pub provider_payment_id: Option<String>,
    /// Unique identifier of the customer within the payment provider.
    pub provider_customer_id: Option<String>,
    /// The next action to be taken by the customer to complete the payment.
    pub next_action: Option<Value>,
    /// When the payment was created.
    pub created_at: DateTime<Utc>,
}

/// The type of the paid resource.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
pub enum PayableType {
    /// An invoice payment.
    Invoice,
    /// A payment request.
    PaymentRequest,
}

/// The normalized payment status by Lago.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PaymentStatus {
    /// Payment succeeded.
    Succeeded,
    /// Payment failed.
    Failed,
    /// Payment is pending.
    Pending,
    /// Payment is being processed.
    Processing,
}

/// The type of payment.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PaymentType {
    /// Manual payment.
    Manual,
    /// Payment through a provider.
    Provider,
}

/// The type of payment provider.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PaymentProviderType {
    /// Adyen payment provider.
    Adyen,
    /// Cashfree payment provider.
    Cashfree,
    /// GoCardless payment provider.
    Gocardless,
    /// Stripe payment provider.
    Stripe,
    /// Flutterwave payment provider.
    Flutterwave,
    /// MoneyHash payment provider.
    Moneyhash,
}
