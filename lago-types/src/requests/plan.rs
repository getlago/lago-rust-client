use serde::{Deserialize, Serialize};

use crate::filters::common::ListFilters;
use crate::filters::plan::PlanFilters;
use crate::models::{ChargeModel, PaginationParams, PlanInterval};

/// Request parameters for listing plans.
#[derive(Debug, Clone)]
pub struct ListPlansRequest {
    pub pagination: PaginationParams,
    pub filters: PlanFilters,
}

impl ListPlansRequest {
    /// Creates a new empty list plans request.
    pub fn new() -> Self {
        Self {
            pagination: PaginationParams::default(),
            filters: PlanFilters::default(),
        }
    }

    /// Sets the pagination parameters for the request.
    pub fn with_pagination(mut self, pagination: PaginationParams) -> Self {
        self.pagination = pagination;
        self
    }

    /// Sets the plan filters for the request.
    pub fn with_filters(mut self, filters: PlanFilters) -> Self {
        self.filters = filters;
        self
    }

    /// Converts the request parameters into HTTP query parameters.
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = self.pagination.to_query_params();
        params.extend(self.filters.to_query_params());
        params
    }
}

impl Default for ListPlansRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Request parameters for retrieving a specific plan.
#[derive(Debug, Clone)]
pub struct GetPlanRequest {
    /// The unique code of the plan.
    pub code: String,
}

impl GetPlanRequest {
    /// Creates a new get plan request.
    pub fn new(code: String) -> Self {
        Self { code }
    }
}

/// Input data for creating a plan charge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlanChargeInput {
    /// The billable metric ID to reference.
    pub billable_metric_id: String,
    /// The charge model to use.
    pub charge_model: ChargeModel,
    /// Whether the charge is invoiceable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoiceable: Option<bool>,
    /// Display name for the charge on invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_display_name: Option<String>,
    /// Whether the charge is billed in advance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_in_advance: Option<bool>,
    /// Whether the charge is prorated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorated: Option<bool>,
    /// Minimum amount in cents for this charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_amount_cents: Option<i64>,
    /// Charge properties (model-specific configuration).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    /// Tax codes for this charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_codes: Option<Vec<String>>,
    /// Filters for differentiated pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<CreateChargeFilterInput>>,
    /// The regroup paid fees option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regroup_paid_fees: Option<String>,
}

impl CreatePlanChargeInput {
    /// Creates a new plan charge input.
    pub fn new(billable_metric_id: String, charge_model: ChargeModel) -> Self {
        Self {
            billable_metric_id,
            charge_model,
            invoiceable: None,
            invoice_display_name: None,
            pay_in_advance: None,
            prorated: None,
            min_amount_cents: None,
            properties: None,
            tax_codes: None,
            filters: None,
            regroup_paid_fees: None,
        }
    }

    /// Sets the invoiceable flag.
    pub fn with_invoiceable(mut self, invoiceable: bool) -> Self {
        self.invoiceable = Some(invoiceable);
        self
    }

    /// Sets the invoice display name.
    pub fn with_invoice_display_name(mut self, name: String) -> Self {
        self.invoice_display_name = Some(name);
        self
    }

    /// Sets the pay in advance flag.
    pub fn with_pay_in_advance(mut self, pay_in_advance: bool) -> Self {
        self.pay_in_advance = Some(pay_in_advance);
        self
    }

    /// Sets the prorated flag.
    pub fn with_prorated(mut self, prorated: bool) -> Self {
        self.prorated = Some(prorated);
        self
    }

    /// Sets the minimum amount in cents.
    pub fn with_min_amount_cents(mut self, min_amount_cents: i64) -> Self {
        self.min_amount_cents = Some(min_amount_cents);
        self
    }

    /// Sets the charge properties.
    pub fn with_properties(mut self, properties: serde_json::Value) -> Self {
        self.properties = Some(properties);
        self
    }

    /// Sets the tax codes.
    pub fn with_tax_codes(mut self, tax_codes: Vec<String>) -> Self {
        self.tax_codes = Some(tax_codes);
        self
    }

    /// Sets the filters.
    pub fn with_filters(mut self, filters: Vec<CreateChargeFilterInput>) -> Self {
        self.filters = Some(filters);
        self
    }
}

/// Input data for creating a charge filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChargeFilterInput {
    /// Invoice display name for this filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_display_name: Option<String>,
    /// Filter properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    /// Filter values mapping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<serde_json::Value>,
}

/// Input data for creating a minimum commitment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMinimumCommitmentInput {
    /// Minimum commitment amount in cents.
    pub amount_cents: i64,
    /// Invoice display name for the minimum commitment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_display_name: Option<String>,
    /// Tax codes for the minimum commitment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_codes: Option<Vec<String>>,
}

impl CreateMinimumCommitmentInput {
    /// Creates a new minimum commitment input.
    pub fn new(amount_cents: i64) -> Self {
        Self {
            amount_cents,
            invoice_display_name: None,
            tax_codes: None,
        }
    }

    /// Sets the invoice display name.
    pub fn with_invoice_display_name(mut self, name: String) -> Self {
        self.invoice_display_name = Some(name);
        self
    }

    /// Sets the tax codes.
    pub fn with_tax_codes(mut self, tax_codes: Vec<String>) -> Self {
        self.tax_codes = Some(tax_codes);
        self
    }
}

/// Input data for creating a usage threshold.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUsageThresholdInput {
    /// Threshold amount in cents.
    pub amount_cents: i64,
    /// Display name for the threshold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_display_name: Option<String>,
    /// Whether the threshold is recurring.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<bool>,
}

impl CreateUsageThresholdInput {
    /// Creates a new usage threshold input.
    pub fn new(amount_cents: i64) -> Self {
        Self {
            amount_cents,
            threshold_display_name: None,
            recurring: None,
        }
    }

    /// Sets the threshold display name.
    pub fn with_threshold_display_name(mut self, name: String) -> Self {
        self.threshold_display_name = Some(name);
        self
    }

    /// Sets the recurring flag.
    pub fn with_recurring(mut self, recurring: bool) -> Self {
        self.recurring = Some(recurring);
        self
    }
}

/// Input data for creating a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlanInput {
    /// Name of the plan.
    pub name: String,
    /// Unique code for the plan.
    pub code: String,
    /// Billing interval.
    pub interval: PlanInterval,
    /// Base amount in cents.
    pub amount_cents: i64,
    /// Currency for the amount.
    pub amount_currency: String,
    /// Display name for invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_display_name: Option<String>,
    /// Description of the plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Trial period in days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period: Option<f64>,
    /// Whether the plan is billed in advance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_in_advance: Option<bool>,
    /// Whether charges are billed monthly for yearly plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_charges_monthly: Option<bool>,
    /// Tax codes for this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_codes: Option<Vec<String>>,
    /// Charges for this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges: Option<Vec<CreatePlanChargeInput>>,
    /// Minimum commitment for this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_commitment: Option<CreateMinimumCommitmentInput>,
    /// Usage thresholds for progressive billing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_thresholds: Option<Vec<CreateUsageThresholdInput>>,
}

impl CreatePlanInput {
    /// Creates a new plan input with required fields.
    pub fn new(
        name: String,
        code: String,
        interval: PlanInterval,
        amount_cents: i64,
        amount_currency: String,
    ) -> Self {
        Self {
            name,
            code,
            interval,
            amount_cents,
            amount_currency,
            invoice_display_name: None,
            description: None,
            trial_period: None,
            pay_in_advance: None,
            bill_charges_monthly: None,
            tax_codes: None,
            charges: None,
            minimum_commitment: None,
            usage_thresholds: None,
        }
    }

    /// Sets the invoice display name.
    pub fn with_invoice_display_name(mut self, name: String) -> Self {
        self.invoice_display_name = Some(name);
        self
    }

    /// Sets the description.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the trial period in days.
    pub fn with_trial_period(mut self, days: f64) -> Self {
        self.trial_period = Some(days);
        self
    }

    /// Sets whether the plan is billed in advance.
    pub fn with_pay_in_advance(mut self, pay_in_advance: bool) -> Self {
        self.pay_in_advance = Some(pay_in_advance);
        self
    }

    /// Sets whether charges are billed monthly.
    pub fn with_bill_charges_monthly(mut self, bill_charges_monthly: bool) -> Self {
        self.bill_charges_monthly = Some(bill_charges_monthly);
        self
    }

    /// Sets the tax codes.
    pub fn with_tax_codes(mut self, tax_codes: Vec<String>) -> Self {
        self.tax_codes = Some(tax_codes);
        self
    }

    /// Sets the charges.
    pub fn with_charges(mut self, charges: Vec<CreatePlanChargeInput>) -> Self {
        self.charges = Some(charges);
        self
    }

    /// Sets the minimum commitment.
    pub fn with_minimum_commitment(
        mut self,
        minimum_commitment: CreateMinimumCommitmentInput,
    ) -> Self {
        self.minimum_commitment = Some(minimum_commitment);
        self
    }

    /// Sets the usage thresholds.
    pub fn with_usage_thresholds(
        mut self,
        usage_thresholds: Vec<CreateUsageThresholdInput>,
    ) -> Self {
        self.usage_thresholds = Some(usage_thresholds);
        self
    }
}

/// Request for creating a plan.
#[derive(Debug, Clone, Serialize)]
pub struct CreatePlanRequest {
    pub plan: CreatePlanInput,
}

impl CreatePlanRequest {
    /// Creates a new create plan request.
    pub fn new(input: CreatePlanInput) -> Self {
        Self { plan: input }
    }
}

/// Input data for updating a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePlanInput {
    /// Name of the plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Unique code for the plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Billing interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<PlanInterval>,
    /// Base amount in cents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i64>,
    /// Currency for the amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_currency: Option<String>,
    /// Display name for invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_display_name: Option<String>,
    /// Description of the plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Trial period in days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period: Option<f64>,
    /// Whether the plan is billed in advance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_in_advance: Option<bool>,
    /// Whether charges are billed monthly for yearly plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_charges_monthly: Option<bool>,
    /// Tax codes for this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_codes: Option<Vec<String>>,
    /// Charges for this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges: Option<Vec<CreatePlanChargeInput>>,
    /// Minimum commitment for this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_commitment: Option<CreateMinimumCommitmentInput>,
    /// Usage thresholds for progressive billing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_thresholds: Option<Vec<CreateUsageThresholdInput>>,
    /// Whether to cascade updates to existing subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascade_updates: Option<bool>,
}

impl UpdatePlanInput {
    /// Creates a new empty update plan input.
    pub fn new() -> Self {
        Self {
            name: None,
            code: None,
            interval: None,
            amount_cents: None,
            amount_currency: None,
            invoice_display_name: None,
            description: None,
            trial_period: None,
            pay_in_advance: None,
            bill_charges_monthly: None,
            tax_codes: None,
            charges: None,
            minimum_commitment: None,
            usage_thresholds: None,
            cascade_updates: None,
        }
    }

    /// Sets the name.
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the code.
    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    /// Sets the interval.
    pub fn with_interval(mut self, interval: PlanInterval) -> Self {
        self.interval = Some(interval);
        self
    }

    /// Sets the amount in cents.
    pub fn with_amount_cents(mut self, amount_cents: i64) -> Self {
        self.amount_cents = Some(amount_cents);
        self
    }

    /// Sets the currency.
    pub fn with_amount_currency(mut self, currency: String) -> Self {
        self.amount_currency = Some(currency);
        self
    }

    /// Sets the invoice display name.
    pub fn with_invoice_display_name(mut self, name: String) -> Self {
        self.invoice_display_name = Some(name);
        self
    }

    /// Sets the description.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the trial period in days.
    pub fn with_trial_period(mut self, days: f64) -> Self {
        self.trial_period = Some(days);
        self
    }

    /// Sets whether the plan is billed in advance.
    pub fn with_pay_in_advance(mut self, pay_in_advance: bool) -> Self {
        self.pay_in_advance = Some(pay_in_advance);
        self
    }

    /// Sets whether charges are billed monthly.
    pub fn with_bill_charges_monthly(mut self, bill_charges_monthly: bool) -> Self {
        self.bill_charges_monthly = Some(bill_charges_monthly);
        self
    }

    /// Sets the tax codes.
    pub fn with_tax_codes(mut self, tax_codes: Vec<String>) -> Self {
        self.tax_codes = Some(tax_codes);
        self
    }

    /// Sets the charges.
    pub fn with_charges(mut self, charges: Vec<CreatePlanChargeInput>) -> Self {
        self.charges = Some(charges);
        self
    }

    /// Sets the minimum commitment.
    pub fn with_minimum_commitment(
        mut self,
        minimum_commitment: CreateMinimumCommitmentInput,
    ) -> Self {
        self.minimum_commitment = Some(minimum_commitment);
        self
    }

    /// Sets the usage thresholds.
    pub fn with_usage_thresholds(
        mut self,
        usage_thresholds: Vec<CreateUsageThresholdInput>,
    ) -> Self {
        self.usage_thresholds = Some(usage_thresholds);
        self
    }

    /// Sets whether to cascade updates to existing subscriptions.
    pub fn with_cascade_updates(mut self, cascade_updates: bool) -> Self {
        self.cascade_updates = Some(cascade_updates);
        self
    }
}

impl Default for UpdatePlanInput {
    fn default() -> Self {
        Self::new()
    }
}

/// Request for updating a plan.
#[derive(Debug, Clone, Serialize)]
pub struct UpdatePlanRequest {
    /// The code of the plan to update.
    #[serde(skip)]
    pub code: String,
    /// The plan update data.
    pub plan: UpdatePlanInput,
}

impl UpdatePlanRequest {
    /// Creates a new update plan request.
    pub fn new(code: String, input: UpdatePlanInput) -> Self {
        Self { code, plan: input }
    }
}

/// Request for deleting a plan.
#[derive(Debug, Clone)]
pub struct DeletePlanRequest {
    /// The code of the plan to delete.
    pub code: String,
}

impl DeletePlanRequest {
    /// Creates a new delete plan request.
    pub fn new(code: String) -> Self {
        Self { code }
    }
}
