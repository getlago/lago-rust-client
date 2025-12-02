use serde::{Deserialize, Serialize};

use crate::models::{PaginationMeta, Plan};

/// Response for retrieving a single plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPlanResponse {
    pub plan: Plan,
}

/// Response for creating a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlanResponse {
    pub plan: Plan,
}

/// Response for updating a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePlanResponse {
    pub plan: Plan,
}

/// Response for deleting a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePlanResponse {
    pub plan: Plan,
}

/// Response for listing plans.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPlansResponse {
    pub plans: Vec<Plan>,
    pub meta: PaginationMeta,
}
