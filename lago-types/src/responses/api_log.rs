use crate::models::{ApiLogObject, PaginationMeta};
use serde::{Deserialize, Serialize};

/// Response containing a list of API logs with pagination metadata.
///
/// This struct represents the API response for API log listing requests,
/// including both the API log data and pagination information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListApiLogsResponse {
    pub api_logs: Vec<ApiLogObject>,
    pub meta: PaginationMeta,
}

/// Response containing a single API log.
///
/// This struct represents the API response for retrieving a specific
/// API log by its identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApiLogResponse {
    pub api_log: ApiLogObject,
}
