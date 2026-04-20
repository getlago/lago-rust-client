use serde::{Deserialize, Serialize};

use crate::models::{ChargeModel, FixedChargeModel, PaginationParams};

// ─── Charge input types ──────────────────────────────────────────────────────

/// Input data for creating a standalone charge on a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChargeInput {
    /// The billable metric ID to reference.
    pub billable_metric_id: String,
    /// The charge model to use.
    pub charge_model: ChargeModel,
    /// Unique code for the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Display name for the charge on invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_display_name: Option<String>,
    /// Whether the charge is billed in advance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_in_advance: Option<bool>,
    /// Whether the charge is invoiceable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoiceable: Option<bool>,
    /// Whether the charge is prorated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorated: Option<bool>,
    /// Minimum amount in cents for this charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_amount_cents: Option<i64>,
    /// Charge properties (model-specific configuration).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    /// Filters for differentiated pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ChargeFilterInputValue>>,
    /// Tax codes for this charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_codes: Option<Vec<String>>,
    /// The regroup paid fees option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regroup_paid_fees: Option<String>,
    /// Applied pricing unit for this charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_pricing_unit: Option<serde_json::Value>,
    /// Whether the charge accepts a target wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepts_target_wallet: Option<bool>,
    /// Whether to cascade updates to existing subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascade_updates: Option<bool>,
}

impl CreateChargeInput {
    pub fn new(billable_metric_id: String, charge_model: ChargeModel) -> Self {
        Self {
            billable_metric_id,
            charge_model,
            code: None,
            invoice_display_name: None,
            pay_in_advance: None,
            invoiceable: None,
            prorated: None,
            min_amount_cents: None,
            properties: None,
            filters: None,
            tax_codes: None,
            regroup_paid_fees: None,
            applied_pricing_unit: None,
            accepts_target_wallet: None,
            cascade_updates: None,
        }
    }

    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    pub fn with_properties(mut self, properties: serde_json::Value) -> Self {
        self.properties = Some(properties);
        self
    }

    pub fn with_tax_codes(mut self, tax_codes: Vec<String>) -> Self {
        self.tax_codes = Some(tax_codes);
        self
    }
}

/// Input data for updating a standalone charge.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChargeInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_in_advance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoiceable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_amount_cents: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ChargeFilterInputValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_codes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_pricing_unit: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepts_target_wallet: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascade_updates: Option<bool>,
}

impl UpdateChargeInput {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Filter values used in charge input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeFilterInputValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascade_updates: Option<bool>,
}

// ─── Fixed charge input types ────────────────────────────────────────────────

/// Input data for creating a standalone fixed charge on a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFixedChargeInput {
    /// The add-on ID to reference.
    pub add_on_id: String,
    /// The charge model to use.
    pub charge_model: FixedChargeModel,
    /// Unique code for the fixed charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Display name for the fixed charge on invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_display_name: Option<String>,
    /// Whether the fixed charge is billed in advance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_in_advance: Option<bool>,
    /// Whether the fixed charge is prorated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorated: Option<bool>,
    /// Number of units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<f64>,
    /// Whether to apply units immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_units_immediately: Option<bool>,
    /// Fixed charge properties (model-specific configuration).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    /// Tax codes for this fixed charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_codes: Option<Vec<String>>,
    /// Whether to cascade updates to existing subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascade_updates: Option<bool>,
}

impl CreateFixedChargeInput {
    pub fn new(add_on_id: String, charge_model: FixedChargeModel) -> Self {
        Self {
            add_on_id,
            charge_model,
            code: None,
            invoice_display_name: None,
            pay_in_advance: None,
            prorated: None,
            units: None,
            apply_units_immediately: None,
            properties: None,
            tax_codes: None,
            cascade_updates: None,
        }
    }

    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    pub fn with_units(mut self, units: f64) -> Self {
        self.units = Some(units);
        self
    }
}

/// Input data for updating a standalone fixed charge.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateFixedChargeInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_units_immediately: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_codes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascade_updates: Option<bool>,
}

impl UpdateFixedChargeInput {
    pub fn new() -> Self {
        Self::default()
    }
}

// ─── Charge filter input types ───────────────────────────────────────────────

/// Input data for creating/updating a standalone charge filter.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChargeFilterInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascade_updates: Option<bool>,
}

impl ChargeFilterInput {
    pub fn new() -> Self {
        Self::default()
    }
}

// ─── Plan charge requests ────────────────────────────────────────────────────

/// Request for listing charges on a plan.
#[derive(Debug, Clone)]
pub struct ListPlanChargesRequest {
    pub plan_code: String,
    pub pagination: PaginationParams,
}

impl ListPlanChargesRequest {
    pub fn new(plan_code: String) -> Self {
        Self {
            plan_code,
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

/// Request for retrieving a specific charge on a plan.
#[derive(Debug, Clone)]
pub struct GetPlanChargeRequest {
    pub plan_code: String,
    pub charge_code: String,
}

impl GetPlanChargeRequest {
    pub fn new(plan_code: String, charge_code: String) -> Self {
        Self {
            plan_code,
            charge_code,
        }
    }
}

/// Request for creating a charge on a plan.
#[derive(Debug, Clone, Serialize)]
pub struct CreatePlanChargeRequest {
    #[serde(skip)]
    pub plan_code: String,
    pub charge: CreateChargeInput,
}

impl CreatePlanChargeRequest {
    pub fn new(plan_code: String, input: CreateChargeInput) -> Self {
        Self {
            plan_code,
            charge: input,
        }
    }
}

/// Request for updating a charge on a plan.
#[derive(Debug, Clone, Serialize)]
pub struct UpdatePlanChargeRequest {
    #[serde(skip)]
    pub plan_code: String,
    #[serde(skip)]
    pub charge_code: String,
    pub charge: UpdateChargeInput,
}

impl UpdatePlanChargeRequest {
    pub fn new(plan_code: String, charge_code: String, input: UpdateChargeInput) -> Self {
        Self {
            plan_code,
            charge_code,
            charge: input,
        }
    }
}

/// Request for deleting a charge on a plan.
#[derive(Debug, Clone)]
pub struct DeletePlanChargeRequest {
    pub plan_code: String,
    pub charge_code: String,
    pub cascade_updates: Option<bool>,
}

impl DeletePlanChargeRequest {
    pub fn new(plan_code: String, charge_code: String) -> Self {
        Self {
            plan_code,
            charge_code,
            cascade_updates: None,
        }
    }

    pub fn with_cascade_updates(mut self, cascade_updates: bool) -> Self {
        self.cascade_updates = Some(cascade_updates);
        self
    }

    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(cascade) = self.cascade_updates {
            params.push(("cascade_updates", cascade.to_string()));
        }
        params
    }
}

// ─── Plan fixed charge requests ──────────────────────────────────────────────

/// Request for listing fixed charges on a plan.
#[derive(Debug, Clone)]
pub struct ListPlanFixedChargesRequest {
    pub plan_code: String,
    pub pagination: PaginationParams,
}

impl ListPlanFixedChargesRequest {
    pub fn new(plan_code: String) -> Self {
        Self {
            plan_code,
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

/// Request for retrieving a specific fixed charge on a plan.
#[derive(Debug, Clone)]
pub struct GetPlanFixedChargeRequest {
    pub plan_code: String,
    pub fixed_charge_code: String,
}

impl GetPlanFixedChargeRequest {
    pub fn new(plan_code: String, fixed_charge_code: String) -> Self {
        Self {
            plan_code,
            fixed_charge_code,
        }
    }
}

/// Request for creating a fixed charge on a plan.
#[derive(Debug, Clone, Serialize)]
pub struct CreatePlanFixedChargeRequest {
    #[serde(skip)]
    pub plan_code: String,
    pub fixed_charge: CreateFixedChargeInput,
}

impl CreatePlanFixedChargeRequest {
    pub fn new(plan_code: String, input: CreateFixedChargeInput) -> Self {
        Self {
            plan_code,
            fixed_charge: input,
        }
    }
}

/// Request for updating a fixed charge on a plan.
#[derive(Debug, Clone, Serialize)]
pub struct UpdatePlanFixedChargeRequest {
    #[serde(skip)]
    pub plan_code: String,
    #[serde(skip)]
    pub fixed_charge_code: String,
    pub fixed_charge: UpdateFixedChargeInput,
}

impl UpdatePlanFixedChargeRequest {
    pub fn new(
        plan_code: String,
        fixed_charge_code: String,
        input: UpdateFixedChargeInput,
    ) -> Self {
        Self {
            plan_code,
            fixed_charge_code,
            fixed_charge: input,
        }
    }
}

/// Request for deleting a fixed charge on a plan.
#[derive(Debug, Clone)]
pub struct DeletePlanFixedChargeRequest {
    pub plan_code: String,
    pub fixed_charge_code: String,
    pub cascade_updates: Option<bool>,
}

impl DeletePlanFixedChargeRequest {
    pub fn new(plan_code: String, fixed_charge_code: String) -> Self {
        Self {
            plan_code,
            fixed_charge_code,
            cascade_updates: None,
        }
    }

    pub fn with_cascade_updates(mut self, cascade_updates: bool) -> Self {
        self.cascade_updates = Some(cascade_updates);
        self
    }

    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(cascade) = self.cascade_updates {
            params.push(("cascade_updates", cascade.to_string()));
        }
        params
    }
}

// ─── Plan charge filter requests ─────────────────────────────────────────────

/// Request for listing charge filters on a plan charge.
#[derive(Debug, Clone)]
pub struct ListPlanChargeFiltersRequest {
    pub plan_code: String,
    pub charge_code: String,
    pub pagination: PaginationParams,
}

impl ListPlanChargeFiltersRequest {
    pub fn new(plan_code: String, charge_code: String) -> Self {
        Self {
            plan_code,
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

/// Request for retrieving a specific charge filter.
#[derive(Debug, Clone)]
pub struct GetPlanChargeFilterRequest {
    pub plan_code: String,
    pub charge_code: String,
    pub filter_id: String,
}

impl GetPlanChargeFilterRequest {
    pub fn new(plan_code: String, charge_code: String, filter_id: String) -> Self {
        Self {
            plan_code,
            charge_code,
            filter_id,
        }
    }
}

/// Request for creating a charge filter on a plan charge.
#[derive(Debug, Clone, Serialize)]
pub struct CreatePlanChargeFilterRequest {
    #[serde(skip)]
    pub plan_code: String,
    #[serde(skip)]
    pub charge_code: String,
    pub filter: ChargeFilterInput,
}

impl CreatePlanChargeFilterRequest {
    pub fn new(plan_code: String, charge_code: String, input: ChargeFilterInput) -> Self {
        Self {
            plan_code,
            charge_code,
            filter: input,
        }
    }
}

/// Request for updating a charge filter on a plan charge.
#[derive(Debug, Clone, Serialize)]
pub struct UpdatePlanChargeFilterRequest {
    #[serde(skip)]
    pub plan_code: String,
    #[serde(skip)]
    pub charge_code: String,
    #[serde(skip)]
    pub filter_id: String,
    pub filter: ChargeFilterInput,
}

impl UpdatePlanChargeFilterRequest {
    pub fn new(
        plan_code: String,
        charge_code: String,
        filter_id: String,
        input: ChargeFilterInput,
    ) -> Self {
        Self {
            plan_code,
            charge_code,
            filter_id,
            filter: input,
        }
    }
}

/// Request for deleting a charge filter on a plan charge.
#[derive(Debug, Clone)]
pub struct DeletePlanChargeFilterRequest {
    pub plan_code: String,
    pub charge_code: String,
    pub filter_id: String,
    pub cascade_updates: Option<bool>,
}

impl DeletePlanChargeFilterRequest {
    pub fn new(plan_code: String, charge_code: String, filter_id: String) -> Self {
        Self {
            plan_code,
            charge_code,
            filter_id,
            cascade_updates: None,
        }
    }

    pub fn with_cascade_updates(mut self, cascade_updates: bool) -> Self {
        self.cascade_updates = Some(cascade_updates);
        self
    }

    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(cascade) = self.cascade_updates {
            params.push(("cascade_updates", cascade.to_string()));
        }
        params
    }
}
