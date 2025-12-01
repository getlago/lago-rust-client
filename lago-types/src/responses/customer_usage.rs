use crate::models::CustomerUsage;
use serde::{Deserialize, Serialize};

/// Response containing customer current usage data.
///
/// This struct represents the API response for retrieving the current usage
/// data for a customer's subscription within the billing period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomerCurrentUsageResponse {
    pub customer_usage: CustomerUsage,
}
