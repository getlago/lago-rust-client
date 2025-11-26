use crate::models::{ActivityLogObject, PaginationMeta};
use serde::{Deserialize, Serialize};

/// Response containing a list of activity logs with pagination metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListActivityLogsResponse {
    pub activity_logs: Vec<ActivityLogObject>,
    pub meta: PaginationMeta,
}

/// Response containing a single activity log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetActivityLogResponse {
    pub activity_log: ActivityLogObject,
}
