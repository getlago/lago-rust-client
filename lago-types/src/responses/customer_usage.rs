use serde::{Deserialize, Serialize};

use crate::models::CustomerUsage;

/// Response for retrieving a customer's current usage.
///
/// This struct represents the response returned when requesting usage data
/// for a specific customer and subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomerCurrentUsageResponse {
    pub customer_usage: CustomerUsage,
}
