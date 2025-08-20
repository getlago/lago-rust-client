use serde::{Deserialize, Serialize};

use crate::models::{BillableMetric, PaginationMeta};

/// Response for listing billable metrics.
///
/// This struct represents the response returned when requesting a list of
/// billable metrics, including pagination metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBillableMetricsResponse {
    pub billable_metrics: Vec<BillableMetric>,
    pub meta: PaginationMeta,
}

/// Response for retrieving a single billable metric.
///
/// This struct represents the response returned when requesting a specific
/// billable metric by its code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBillableMetricResponse {
    pub billable_metric: BillableMetric,
}

/// Response for creating a billable metric.
///
/// This struct represents the response returned when successfully creating
/// a new billable metric.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBillableMetricResponse {
    pub billable_metric: BillableMetric,
}
