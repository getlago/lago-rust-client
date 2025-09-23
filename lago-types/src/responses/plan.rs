use serde::{Deserialize, Serialize};

use crate::models::{Plan, PaginationMeta};

/// Response for listing plans.
///
/// This struct represents the response returned when requesting a list of
/// plans, including pagination metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPlansResponse {
    pub plans: Vec<Plan>,
    pub meta: PaginationMeta,
}

/// Response for retrieving a single plan.
///
/// This struct represents the response returned when requesting a specific
/// plan by its code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPlanResponse {
    pub plan: Plan,
}

/// Response for creating a plan.
///
/// This struct represents the response returned when successfully creating
/// a new plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlanResponse {
    pub plan: Plan,
}

/// Response for updating a plan.
///
/// This struct represents the response returned when successfully updating
/// an existing plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePlanResponse {
    pub plan: Plan,
}

/// Response for deleting a plan.
///
/// This struct represents the response returned when successfully deleting
/// a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePlanResponse {
    pub plan: Plan,
}