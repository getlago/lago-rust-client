use serde::Serialize;

use crate::models::PaginationParams;
use crate::requests::plan_charge::{ChargeFilterInput, UpdateChargeInput, UpdateFixedChargeInput};

// ─── Subscription charge requests ────────────────────────────────────────────

/// Request for listing charges on a subscription.
#[derive(Debug, Clone)]
pub struct ListSubscriptionChargesRequest {
    pub external_id: String,
    pub pagination: PaginationParams,
}

impl ListSubscriptionChargesRequest {
    pub fn new(external_id: String) -> Self {
        Self {
            external_id,
            pagination: PaginationParams::default(),
        }
    }

    pub fn with_pagination(mut self, pagination: PaginationParams) -> Self {
        self.pagination = pagination;
        self
    }

    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        self.pagination.to_query_params()
    }
}

/// Request for retrieving a specific charge on a subscription.
#[derive(Debug, Clone)]
pub struct GetSubscriptionChargeRequest {
    pub external_id: String,
    pub charge_code: String,
}

impl GetSubscriptionChargeRequest {
    pub fn new(external_id: String, charge_code: String) -> Self {
        Self {
            external_id,
            charge_code,
        }
    }
}

/// Request for updating a charge on a subscription.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateSubscriptionChargeRequest {
    #[serde(skip)]
    pub external_id: String,
    #[serde(skip)]
    pub charge_code: String,
    pub charge: UpdateChargeInput,
}

impl UpdateSubscriptionChargeRequest {
    pub fn new(external_id: String, charge_code: String, input: UpdateChargeInput) -> Self {
        Self {
            external_id,
            charge_code,
            charge: input,
        }
    }
}

// ─── Subscription fixed charge requests ──────────────────────────────────────

/// Request for listing fixed charges on a subscription.
#[derive(Debug, Clone)]
pub struct ListSubscriptionFixedChargesRequest {
    pub external_id: String,
    pub pagination: PaginationParams,
}

impl ListSubscriptionFixedChargesRequest {
    pub fn new(external_id: String) -> Self {
        Self {
            external_id,
            pagination: PaginationParams::default(),
        }
    }

    pub fn with_pagination(mut self, pagination: PaginationParams) -> Self {
        self.pagination = pagination;
        self
    }

    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        self.pagination.to_query_params()
    }
}

/// Request for retrieving a specific fixed charge on a subscription.
#[derive(Debug, Clone)]
pub struct GetSubscriptionFixedChargeRequest {
    pub external_id: String,
    pub fixed_charge_code: String,
}

impl GetSubscriptionFixedChargeRequest {
    pub fn new(external_id: String, fixed_charge_code: String) -> Self {
        Self {
            external_id,
            fixed_charge_code,
        }
    }
}

/// Request for updating a fixed charge on a subscription.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateSubscriptionFixedChargeRequest {
    #[serde(skip)]
    pub external_id: String,
    #[serde(skip)]
    pub fixed_charge_code: String,
    pub fixed_charge: UpdateFixedChargeInput,
}

impl UpdateSubscriptionFixedChargeRequest {
    pub fn new(
        external_id: String,
        fixed_charge_code: String,
        input: UpdateFixedChargeInput,
    ) -> Self {
        Self {
            external_id,
            fixed_charge_code,
            fixed_charge: input,
        }
    }
}

// ─── Subscription charge filter requests ─────────────────────────────────────

/// Request for listing charge filters on a subscription charge.
#[derive(Debug, Clone)]
pub struct ListSubscriptionChargeFiltersRequest {
    pub external_id: String,
    pub charge_code: String,
    pub pagination: PaginationParams,
}

impl ListSubscriptionChargeFiltersRequest {
    pub fn new(external_id: String, charge_code: String) -> Self {
        Self {
            external_id,
            charge_code,
            pagination: PaginationParams::default(),
        }
    }

    pub fn with_pagination(mut self, pagination: PaginationParams) -> Self {
        self.pagination = pagination;
        self
    }

    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        self.pagination.to_query_params()
    }
}

/// Request for retrieving a specific charge filter on a subscription.
#[derive(Debug, Clone)]
pub struct GetSubscriptionChargeFilterRequest {
    pub external_id: String,
    pub charge_code: String,
    pub filter_id: String,
}

impl GetSubscriptionChargeFilterRequest {
    pub fn new(external_id: String, charge_code: String, filter_id: String) -> Self {
        Self {
            external_id,
            charge_code,
            filter_id,
        }
    }
}

/// Request for creating a charge filter on a subscription charge.
#[derive(Debug, Clone, Serialize)]
pub struct CreateSubscriptionChargeFilterRequest {
    #[serde(skip)]
    pub external_id: String,
    #[serde(skip)]
    pub charge_code: String,
    pub filter: ChargeFilterInput,
}

impl CreateSubscriptionChargeFilterRequest {
    pub fn new(external_id: String, charge_code: String, input: ChargeFilterInput) -> Self {
        Self {
            external_id,
            charge_code,
            filter: input,
        }
    }
}

/// Request for updating a charge filter on a subscription charge.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateSubscriptionChargeFilterRequest {
    #[serde(skip)]
    pub external_id: String,
    #[serde(skip)]
    pub charge_code: String,
    #[serde(skip)]
    pub filter_id: String,
    pub filter: ChargeFilterInput,
}

impl UpdateSubscriptionChargeFilterRequest {
    pub fn new(
        external_id: String,
        charge_code: String,
        filter_id: String,
        input: ChargeFilterInput,
    ) -> Self {
        Self {
            external_id,
            charge_code,
            filter_id,
            filter: input,
        }
    }
}

/// Request for deleting a charge filter on a subscription charge.
#[derive(Debug, Clone)]
pub struct DeleteSubscriptionChargeFilterRequest {
    pub external_id: String,
    pub charge_code: String,
    pub filter_id: String,
}

impl DeleteSubscriptionChargeFilterRequest {
    pub fn new(external_id: String, charge_code: String, filter_id: String) -> Self {
        Self {
            external_id,
            charge_code,
            filter_id,
        }
    }
}
