use crate::models::{Fee, PaginationMeta};
use serde::{Deserialize, Serialize};

/// Response containing a list of fees with pagination metadata.
///
/// This struct represents the API response for fee listing requests,
/// including both the fee data and pagination information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFeesResponse {
    pub fees: Vec<Fee>,
    pub meta: PaginationMeta,
}

/// Response containing a single fee.
///
/// This struct represents the API response for retrieving a specific
/// fee by its Lago ID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFeeResponse {
    pub fee: Fee,
}
