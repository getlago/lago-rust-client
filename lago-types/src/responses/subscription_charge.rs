use serde::{Deserialize, Serialize};

use crate::models::{ChargeFilterResponse, FixedCharge, PaginationMeta, PlanCharge};

// ─── Subscription charge responses ───────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscriptionChargeResponse {
    pub charge: PlanCharge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSubscriptionChargesResponse {
    pub charges: Vec<PlanCharge>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubscriptionChargeResponse {
    pub charge: PlanCharge,
}

// ─── Subscription fixed charge responses ─────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscriptionFixedChargeResponse {
    pub fixed_charge: FixedCharge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSubscriptionFixedChargesResponse {
    pub fixed_charges: Vec<FixedCharge>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubscriptionFixedChargeResponse {
    pub fixed_charge: FixedCharge,
}

// ─── Subscription charge filter responses ────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscriptionChargeFilterResponse {
    pub filter: ChargeFilterResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSubscriptionChargeFiltersResponse {
    pub filters: Vec<ChargeFilterResponse>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionChargeFilterResponse {
    pub filter: ChargeFilterResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubscriptionChargeFilterResponse {
    pub filter: ChargeFilterResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSubscriptionChargeFilterResponse {
    pub filter: ChargeFilterResponse,
}
