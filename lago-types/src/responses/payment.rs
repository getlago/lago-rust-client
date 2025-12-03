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
