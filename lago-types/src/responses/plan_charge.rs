use serde::{Deserialize, Serialize};

use crate::models::{ChargeFilterResponse, FixedCharge, PaginationMeta, PlanCharge};

// ─── Plan charge responses ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPlanChargeResponse {
    pub charge: PlanCharge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPlanChargesResponse {
    pub charges: Vec<PlanCharge>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlanChargeResponse {
    pub charge: PlanCharge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePlanChargeResponse {
    pub charge: PlanCharge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePlanChargeResponse {
    pub charge: PlanCharge,
}

// ─── Plan fixed charge responses ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPlanFixedChargeResponse {
    pub fixed_charge: FixedCharge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPlanFixedChargesResponse {
    pub fixed_charges: Vec<FixedCharge>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlanFixedChargeResponse {
    pub fixed_charge: FixedCharge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePlanFixedChargeResponse {
    pub fixed_charge: FixedCharge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePlanFixedChargeResponse {
    pub fixed_charge: FixedCharge,
}

// ─── Plan charge filter responses ────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPlanChargeFilterResponse {
    pub filter: ChargeFilterResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPlanChargeFiltersResponse {
    pub filters: Vec<ChargeFilterResponse>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlanChargeFilterResponse {
    pub filter: ChargeFilterResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePlanChargeFilterResponse {
    pub filter: ChargeFilterResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePlanChargeFilterResponse {
    pub filter: ChargeFilterResponse,
}
