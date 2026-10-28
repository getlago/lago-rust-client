use crate::models::{PaginationMeta, Payment};
use serde::{Deserialize, Serialize};

/// Response containing a list of payments with pagination metadata.
///
/// This struct represents the API response for payment listing requests,
/// including both the payment data and pagination information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPaymentsResponse {
    pub payments: Vec<Payment>,
    pub meta: PaginationMeta,
}

/// Response containing a single payment.
///
/// This struct represents the API response for retrieving a specific
/// payment by its identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPaymentResponse {
    pub payment: Payment,
}

/// Response containing a created payment.
///
/// This struct represents the API response for creating a payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentResponse {
    pub payment: Payment,
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAYMENT_JSON: &str = r#"{
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
    }"#;

    #[test]
    fn get_payment_response_deserializes_invoice_numbers() {
        let json = format!(r#"{{"payment": {PAYMENT_JSON}}}"#);
        let response: GetPaymentResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(
            response.payment.invoice_numbers,
            vec!["LAG-1234-001-001".to_string()]
        );
    }

    #[test]
    fn create_payment_response_deserializes_invoice_numbers() {
        let json = format!(r#"{{"payment": {PAYMENT_JSON}}}"#);
        let response: CreatePaymentResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(
            response.payment.invoice_numbers,
            vec!["LAG-1234-001-001".to_string()]
        );
    }

    #[test]
    fn list_payments_response_deserializes_invoice_numbers() {
        let json = format!(
            r#"{{
                "payments": [{PAYMENT_JSON}],
                "meta": {{
                    "current_page": 1,
                    "next_page": null,
                    "prev_page": null,
                    "total_pages": 1,
                    "total_count": 1
                }}
            }}"#
        );
        let response: ListPaymentsResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response.payments.len(), 1);
        assert_eq!(
            response.payments[0].invoice_numbers,
            vec!["LAG-1234-001-001".to_string()]
        );
    }
}
