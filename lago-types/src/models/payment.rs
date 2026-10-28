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
    /// List of invoice numbers associated with the payment.
    pub invoice_numbers: Vec<String>,
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
    #[serde(alias = "PaymentProviders::AdyenProvider")]
    Adyen,
    /// Cashfree payment provider.
    #[serde(alias = "PaymentProviders::CashfreeProvider")]
    Cashfree,
    /// GoCardless payment provider.
    #[serde(alias = "PaymentProviders::GocardlessProvider")]
    Gocardless,
    /// Stripe payment provider.
    #[serde(alias = "PaymentProviders::StripeProvider")]
    Stripe,
    /// Flutterwave payment provider.
    #[serde(alias = "PaymentProviders::FlutterwaveProvider")]
    Flutterwave,
    /// MoneyHash payment provider.
    #[serde(alias = "PaymentProviders::MoneyhashProvider")]
    Moneyhash,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn payment_json() -> String {
        r#"{
            "lago_id": "3c123c12-3c12-3c12-3c12-3c123c123c12",
            "lago_customer_id": "2b012b01-2b01-2b01-2b01-2b012b012b01",
            "external_customer_id": "customer_123",
            "invoice_ids": ["1a901a90-1a90-1a90-1a90-1a901a901a90"],
            "invoice_numbers": ["LAG-1234-001-001"],
            "lago_payable_id": "1a901a90-1a90-1a90-1a90-1a901a901a90",
            "payable_type": "Invoice",
            "amount_cents": 120,
            "amount_currency": "EUR",
            "status": "succeeded",
            "payment_status": "succeeded",
            "type": "manual",
            "created_at": "2026-10-01T00:00:00Z"
        }"#
        .to_string()
    }

    #[test]
    fn deserializes_invoice_numbers() {
        let payment: Payment = serde_json::from_str(&payment_json()).unwrap();

        assert_eq!(
            payment.invoice_numbers,
            vec!["LAG-1234-001-001".to_string()]
        );
        assert_eq!(
            payment.invoice_ids,
            vec![Uuid::parse_str("1a901a90-1a90-1a90-1a90-1a901a901a90").unwrap()]
        );
    }

    #[test]
    fn deserializes_empty_invoice_numbers() {
        let json = payment_json().replace(
            r#""invoice_numbers": ["LAG-1234-001-001"],"#,
            r#""invoice_numbers": [],"#,
        );
        let payment: Payment = serde_json::from_str(&json).unwrap();

        assert!(payment.invoice_numbers.is_empty());
    }

    #[test]
    fn rejects_payment_without_invoice_numbers() {
        let json = payment_json().replace(r#""invoice_numbers": ["LAG-1234-001-001"],"#, "");
        let error = serde_json::from_str::<Payment>(&json).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("missing field `invoice_numbers`")
        );
    }
}
