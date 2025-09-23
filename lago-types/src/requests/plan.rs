use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::{ChargeModel, PaginationParams, PlanInterval, RegroupPaidFeesType};

/// Filter parameters for plan list operations.
///
/// Currently provides a minimal but extensible structure for future
/// plan filtering capabilities.
#[derive(Debug, Clone, Default)]
pub struct PlanFilter {
    // Currently, the API doesn't support many filters for plans
    // We're keeping this minimal but extensible structure for future additions
}

impl PlanFilter {
    /// Create a new empty plan filter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Converts the plan filter into HTTP query parameters.
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        Vec::new()
    }
}

/// Request parameters for listing plans.
///
/// Combines pagination parameters with plan-specific filters to build
/// a comprehensive request for retrieving plan lists from the API.
#[derive(Debug, Clone)]
pub struct ListPlansRequest {
    pub pagination: PaginationParams,
    pub filters: PlanFilter,
}

impl ListPlansRequest {
    /// Creates a new empty list plans request.
    ///
    /// # Returns
    /// A new `ListPlansRequest` instance with default pagination and no filters.
    pub fn new() -> Self {
        Self {
            pagination: PaginationParams::default(),
            filters: PlanFilter::default(),
        }
    }

    /// Sets the pagination parameters for the request.
    ///
    /// # Arguments
    /// * `pagination` - The pagination parameters to use
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_pagination(mut self, pagination: PaginationParams) -> Self {
        self.pagination = pagination;
        self
    }

    /// Sets the plan filters for the request.
    ///
    /// # Arguments
    /// * `filters` - The plan filters to apply
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_filters(mut self, filters: PlanFilter) -> Self {
        self.filters = filters;
        self
    }

    /// Converts the request parameters into HTTP query parameters.
    ///
    /// # Returns
    /// A vector of query parameter tuples containing both pagination and filter criteria.
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
///
/// Used to fetch a single plan by its unique code identifier.
#[derive(Debug, Clone)]
pub struct GetPlanRequest {
    pub code: String,
}

impl GetPlanRequest {
    /// Creates a new get plan request.
    ///
    /// # Arguments
    /// * `code` - The code of the plan to retrieve
    ///
    /// # Returns
    /// A new `GetPlanRequest` instance
    pub fn new(code: String) -> Self {
        Self { code }
    }
}

/// Request parameters for creating a new plan.
///
/// Contains all the necessary information to create a comprehensive billing plan
/// in the Lago system, including base pricing, charges, commitments, and thresholds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlanInput {
    pub name: String,
    pub code: String,
    pub interval: PlanInterval,
    pub description: Option<String>,
    pub amount_cents: i64,
    pub amount_currency: String,
    pub trial_period: Option<f64>,
    pub pay_in_advance: Option<bool>,
    pub invoiceable: Option<bool>,
    pub bill_charges_monthly: Option<bool>,
    pub invoice_display_name: Option<String>,
    pub tax_codes: Option<Vec<String>>,
    pub minimum_commitment: Option<CreateMinimumCommitmentInput>,
    pub charges: Option<Vec<CreateChargeInput>>,
    pub usage_thresholds: Option<Vec<CreateUsageThresholdInput>>,
}

impl CreatePlanInput {
    /// Creates a new plan input with required fields.
    ///
    /// # Arguments
    /// * `name` - The name of the plan
    /// * `code` - The unique code for the plan
    /// * `interval` - The billing interval
    /// * `amount_cents` - The base cost in cents
    /// * `amount_currency` - The currency code
    ///
    /// # Returns
    /// A new `CreatePlanInput` instance
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
            description: None,
            trial_period: None,
            pay_in_advance: None,
            invoiceable: None,
            bill_charges_monthly: None,
            invoice_display_name: None,
            tax_codes: None,
            minimum_commitment: None,
            charges: None,
            usage_thresholds: None,
        }
    }

    /// Sets the description for the plan.
    ///
    /// # Arguments
    /// * `description` - The description of the plan
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the trial period for the plan.
    ///
    /// # Arguments
    /// * `days` - The number of days for the trial period
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_trial_period(mut self, days: f64) -> Self {
        self.trial_period = Some(days);
        self
    }

    /// Sets whether the plan charges are paid in advance.
    ///
    /// # Arguments
    /// * `pay_in_advance` - Whether charges are paid in advance
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_pay_in_advance(mut self, pay_in_advance: bool) -> Self {
        self.pay_in_advance = Some(pay_in_advance);
        self
    }

    /// Sets whether the plan is invoiceable.
    ///
    /// # Arguments
    /// * `invoiceable` - Whether the plan can generate invoices
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_invoiceable(mut self, invoiceable: bool) -> Self {
        self.invoiceable = Some(invoiceable);
        self
    }

    /// Sets whether charges are billed monthly regardless of plan interval.
    ///
    /// # Arguments
    /// * `bill_monthly` - Whether to bill charges monthly
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_bill_charges_monthly(mut self, bill_monthly: bool) -> Self {
        self.bill_charges_monthly = Some(bill_monthly);
        self
    }

    /// Sets the display name for invoices.
    ///
    /// # Arguments
    /// * `display_name` - The display name to use on invoices
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_invoice_display_name(mut self, display_name: String) -> Self {
        self.invoice_display_name = Some(display_name);
        self
    }

    /// Sets the tax codes for the plan.
    ///
    /// # Arguments
    /// * `tax_codes` - Vector of tax codes to apply to the plan
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_tax_codes(mut self, tax_codes: Vec<String>) -> Self {
        self.tax_codes = Some(tax_codes);
        self
    }

    /// Sets the minimum commitment for the plan.
    ///
    /// # Arguments
    /// * `commitment` - The minimum commitment configuration
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_minimum_commitment(mut self, commitment: CreateMinimumCommitmentInput) -> Self {
        self.minimum_commitment = Some(commitment);
        self
    }

    /// Sets the charges for the plan.
    ///
    /// # Arguments
    /// * `charges` - Vector of charges to include in the plan
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_charges(mut self, charges: Vec<CreateChargeInput>) -> Self {
        self.charges = Some(charges);
        self
    }

    /// Sets the usage thresholds for the plan.
    ///
    /// # Arguments
    /// * `thresholds` - Vector of usage thresholds to apply
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_usage_thresholds(mut self, thresholds: Vec<CreateUsageThresholdInput>) -> Self {
        self.usage_thresholds = Some(thresholds);
        self
    }
}

/// Input for creating minimum commitment.
///
/// Defines the minimum commitment amount that a customer must pay for a plan,
/// regardless of usage. This is useful for ensuring minimum revenue per customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMinimumCommitmentInput {
    pub amount_cents: i64,
    pub invoice_display_name: Option<String>,
    pub tax_codes: Option<Vec<String>>,
}

impl CreateMinimumCommitmentInput {
    /// Creates a new minimum commitment input.
    ///
    /// # Arguments
    /// * `amount_cents` - The minimum commitment amount in cents
    ///
    /// # Returns
    /// A new `CreateMinimumCommitmentInput` instance
    pub fn new(amount_cents: i64) -> Self {
        Self {
            amount_cents,
            invoice_display_name: None,
            tax_codes: None,
        }
    }

    /// Sets the invoice display name for the minimum commitment.
    ///
    /// # Arguments
    /// * `display_name` - The display name to use on invoices
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_invoice_display_name(mut self, display_name: String) -> Self {
        self.invoice_display_name = Some(display_name);
        self
    }

    /// Sets the tax codes for the minimum commitment.
    ///
    /// # Arguments
    /// * `tax_codes` - Vector of tax codes to apply to the commitment
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_tax_codes(mut self, tax_codes: Vec<String>) -> Self {
        self.tax_codes = Some(tax_codes);
        self
    }
}

/// Input for creating a charge.
///
/// A charge defines how a billable metric is priced within a plan. It includes
/// the pricing model, configuration properties, and billing behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChargeInput {
    pub billable_metric_id: Uuid,
    pub charge_model: ChargeModel,
    pub invoice_display_name: Option<String>,
    pub pay_in_advance: Option<bool>,
    pub invoiceable: Option<bool>,
    pub prorated: Option<bool>,
    pub min_amount_cents: Option<i64>,
    pub properties: CreateChargeProperties,
    pub group_properties: Option<Vec<CreateChargeGroupProperty>>,
    pub tax_codes: Option<Vec<String>>,
    pub regroup_paid_fees: Option<RegroupPaidFeesType>,
    pub filters: Option<Vec<CreateChargeFilterInput>>,
}

impl CreateChargeInput {
    /// Creates a new charge input with required fields.
    ///
    /// # Arguments
    /// * `billable_metric_id` - The UUID of the billable metric to charge for
    /// * `charge_model` - The pricing model to use for this charge
    /// * `properties` - The configuration properties for the charge model
    ///
    /// # Returns
    /// A new `CreateChargeInput` instance
    pub fn new(
        billable_metric_id: Uuid,
        charge_model: ChargeModel,
        properties: CreateChargeProperties,
    ) -> Self {
        Self {
            billable_metric_id,
            charge_model,
            properties,
            invoice_display_name: None,
            pay_in_advance: None,
            invoiceable: None,
            prorated: None,
            min_amount_cents: None,
            group_properties: None,
            tax_codes: None,
            regroup_paid_fees: None,
            filters: None,
        }
    }

    /// Sets the invoice display name for the charge.
    ///
    /// # Arguments
    /// * `display_name` - The display name to use on invoices
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_invoice_display_name(mut self, display_name: String) -> Self {
        self.invoice_display_name = Some(display_name);
        self
    }

    /// Sets whether the charge is paid in advance.
    ///
    /// # Arguments
    /// * `pay_in_advance` - Whether the charge is paid in advance
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_pay_in_advance(mut self, pay_in_advance: bool) -> Self {
        self.pay_in_advance = Some(pay_in_advance);
        self
    }

    /// Sets whether the charge is invoiceable.
    ///
    /// # Arguments
    /// * `invoiceable` - Whether the charge appears on invoices
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_invoiceable(mut self, invoiceable: bool) -> Self {
        self.invoiceable = Some(invoiceable);
        self
    }

    /// Sets whether the charge is prorated.
    ///
    /// # Arguments
    /// * `prorated` - Whether to prorate the charge for partial periods
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_prorated(mut self, prorated: bool) -> Self {
        self.prorated = Some(prorated);
        self
    }

    /// Sets the minimum amount for the charge.
    ///
    /// # Arguments
    /// * `min_amount` - The minimum charge amount in cents
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_min_amount_cents(mut self, min_amount: i64) -> Self {
        self.min_amount_cents = Some(min_amount);
        self
    }

    /// Sets group-specific properties for the charge.
    ///
    /// # Arguments
    /// * `group_properties` - Vector of group-specific charge configurations
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_group_properties(
        mut self,
        group_properties: Vec<CreateChargeGroupProperty>,
    ) -> Self {
        self.group_properties = Some(group_properties);
        self
    }

    /// Sets the tax codes for the charge.
    ///
    /// # Arguments
    /// * `tax_codes` - Vector of tax codes to apply to the charge
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_tax_codes(mut self, tax_codes: Vec<String>) -> Self {
        self.tax_codes = Some(tax_codes);
        self
    }

    /// Sets the regroup paid fees behavior.
    ///
    /// # Arguments
    /// * `regroup_type` - How to handle regrouping of paid fees
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_regroup_paid_fees(mut self, regroup_type: RegroupPaidFeesType) -> Self {
        self.regroup_paid_fees = Some(regroup_type);
        self
    }

    /// Sets the filters for the charge.
    ///
    /// # Arguments
    /// * `filters` - Vector of filters to apply to the charge
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_filters(mut self, filters: Vec<CreateChargeFilterInput>) -> Self {
        self.filters = Some(filters);
        self
    }
}

/// Properties for creating different charge models.
///
/// This enum defines the configuration properties for various pricing models
/// supported by Lago. Each variant corresponds to a specific charge model type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateChargeProperties {
    Standard {
        amount: String,
    },
    Graduated {
        graduated_ranges: Vec<CreateGraduatedRange>,
    },
    GraduatedPercentage {
        graduated_percentage_ranges: Vec<CreateGraduatedPercentageRange>,
    },
    Package {
        amount: String,
        free_units: i64,
        package_size: i64,
    },
    Percentage {
        rate: String,
        fixed_amount: Option<String>,
        free_units_per_events: Option<i32>,
        free_units_per_total_aggregation: Option<String>,
    },
    Volume {
        volume_ranges: Vec<CreateVolumeRange>,
    },
    Dynamic {
        #[serde(flatten)]
        custom_properties: HashMap<String, serde_json::Value>,
    },
}

/// Input for creating graduated range.
///
/// Defines a tier in graduated pricing where different amounts are charged
/// based on usage ranges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGraduatedRange {
    pub from_value: i64,
    pub to_value: Option<i64>,
    pub flat_amount: String,
    pub per_unit_amount: String,
}

/// Input for creating graduated percentage range.
///
/// Defines a tier in graduated percentage pricing where different percentage
/// rates are applied based on usage ranges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGraduatedPercentageRange {
    pub from_value: i64,
    pub to_value: Option<i64>,
    pub rate: String,
    pub flat_amount: String,
}

/// Input for creating volume range.
///
/// Defines a tier in volume pricing where the entire usage is charged
/// at the rate corresponding to the tier it falls into.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVolumeRange {
    pub from_value: i64,
    pub to_value: Option<i64>,
    pub flat_amount: String,
    pub per_unit_amount: String,
}

/// Input for creating charge group properties.
///
/// Defines group-specific pricing properties for charges when using
/// billable metrics with groups.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChargeGroupProperty {
    pub group_id: String,
    pub values: HashMap<String, String>,
    pub invoice_display_name: Option<String>,
}

/// Input for creating charge filters.
///
/// Defines filters that can be applied to charges to create different
/// pricing rules based on event properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChargeFilterInput {
    pub invoice_display_name: Option<String>,
    pub properties: HashMap<String, serde_json::Value>,
    pub values: HashMap<String, Vec<String>>,
}

/// Input for creating usage thresholds.
///
/// Usage thresholds allow you to define spending limits that trigger
/// notifications or actions when reached during a billing period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUsageThresholdInput {
    pub threshold_display_name: Option<String>,
    pub amount_cents: i64,
    pub recurring: Option<bool>,
}

impl CreateUsageThresholdInput {
    /// Creates a new usage threshold input.
    ///
    /// # Arguments
    /// * `amount_cents` - The threshold amount in cents
    ///
    /// # Returns
    /// A new `CreateUsageThresholdInput` instance
    pub fn new(amount_cents: i64) -> Self {
        Self {
            amount_cents,
            threshold_display_name: None,
            recurring: None,
        }
    }

    /// Sets the display name for the threshold.
    ///
    /// # Arguments
    /// * `display_name` - The display name for the threshold
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_display_name(mut self, display_name: String) -> Self {
        self.threshold_display_name = Some(display_name);
        self
    }

    /// Sets whether the threshold is recurring.
    ///
    /// # Arguments
    /// * `recurring` - Whether the threshold applies to each billing period
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_recurring(mut self, recurring: bool) -> Self {
        self.recurring = Some(recurring);
        self
    }
}

/// Wrapper request for creating a plan.
///
/// This struct wraps the plan input data in the format expected by the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlanRequest {
    pub plan: CreatePlanInput,
}

impl CreatePlanRequest {
    /// Creates a new create plan request.
    ///
    /// # Arguments
    /// * `plan` - The plan input data
    ///
    /// # Returns
    /// A new `CreatePlanRequest` instance
    pub fn new(plan: CreatePlanInput) -> Self {
        Self { plan }
    }
}

/// Update plan input data.
///
/// Contains the fields that can be updated for an existing plan. All fields
/// are optional, allowing for partial updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePlanInput {
    pub name: Option<String>,
    pub invoice_display_name: Option<String>,
    pub description: Option<String>,
    pub tax_codes: Option<Vec<String>>,
    pub minimum_commitment: Option<CreateMinimumCommitmentInput>,
    pub charges: Option<Vec<UpdateChargeInput>>,
    pub usage_thresholds: Option<Vec<CreateUsageThresholdInput>>,
    pub cascade_updates: Option<bool>,
}

impl UpdatePlanInput {
    /// Creates a new empty update plan input.
    ///
    /// # Returns
    /// A new `UpdatePlanInput` instance with all fields set to None
    pub fn new() -> Self {
        Self {
            name: None,
            invoice_display_name: None,
            description: None,
            tax_codes: None,
            minimum_commitment: None,
            charges: None,
            usage_thresholds: None,
            cascade_updates: None,
        }
    }

    /// Sets the plan name.
    ///
    /// # Arguments
    /// * `name` - The new name for the plan
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the invoice display name.
    ///
    /// # Arguments
    /// * `display_name` - The new display name for invoices
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_invoice_display_name(mut self, display_name: String) -> Self {
        self.invoice_display_name = Some(display_name);
        self
    }

    /// Sets the plan description.
    ///
    /// # Arguments
    /// * `description` - The new description for the plan
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the tax codes.
    ///
    /// # Arguments
    /// * `tax_codes` - Vector of tax codes to apply to the plan
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_tax_codes(mut self, tax_codes: Vec<String>) -> Self {
        self.tax_codes = Some(tax_codes);
        self
    }

    /// Sets the minimum commitment.
    ///
    /// # Arguments
    /// * `commitment` - The new minimum commitment configuration
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_minimum_commitment(mut self, commitment: CreateMinimumCommitmentInput) -> Self {
        self.minimum_commitment = Some(commitment);
        self
    }

    /// Sets the charges for the plan.
    ///
    /// # Arguments
    /// * `charges` - Vector of updated charges for the plan
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_charges(mut self, charges: Vec<UpdateChargeInput>) -> Self {
        self.charges = Some(charges);
        self
    }

    /// Sets the usage thresholds.
    ///
    /// # Arguments
    /// * `thresholds` - Vector of usage thresholds to apply
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_usage_thresholds(mut self, thresholds: Vec<CreateUsageThresholdInput>) -> Self {
        self.usage_thresholds = Some(thresholds);
        self
    }

    /// Sets whether to cascade updates to existing subscriptions.
    ///
    /// # Arguments
    /// * `cascade` - Whether to apply changes to existing subscriptions
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_cascade_updates(mut self, cascade: bool) -> Self {
        self.cascade_updates = Some(cascade);
        self
    }
}

impl Default for UpdatePlanInput {
    fn default() -> Self {
        Self::new()
    }
}

/// Input for updating an existing charge.
///
/// Contains the fields that can be updated for an existing charge within a plan.
/// All fields are optional, allowing for partial updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChargeInput {
    pub id: Option<Uuid>,
    pub billable_metric_id: Option<Uuid>,
    pub charge_model: Option<ChargeModel>,
    pub invoice_display_name: Option<String>,
    pub pay_in_advance: Option<bool>,
    pub invoiceable: Option<bool>,
    pub prorated: Option<bool>,
    pub min_amount_cents: Option<i64>,
    pub properties: Option<CreateChargeProperties>,
    pub group_properties: Option<Vec<CreateChargeGroupProperty>>,
    pub tax_codes: Option<Vec<String>>,
    pub regroup_paid_fees: Option<RegroupPaidFeesType>,
    pub filters: Option<Vec<CreateChargeFilterInput>>,
}

impl UpdateChargeInput {
    /// Creates a new empty update charge input.
    ///
    /// # Returns
    /// A new `UpdateChargeInput` instance with all fields set to None
    pub fn new() -> Self {
        Self {
            id: None,
            billable_metric_id: None,
            charge_model: None,
            invoice_display_name: None,
            pay_in_advance: None,
            invoiceable: None,
            prorated: None,
            min_amount_cents: None,
            properties: None,
            group_properties: None,
            tax_codes: None,
            regroup_paid_fees: None,
            filters: None,
        }
    }

    /// Sets the charge ID for updates.
    ///
    /// # Arguments
    /// * `id` - The UUID of the existing charge to update
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the billable metric ID.
    ///
    /// # Arguments
    /// * `billable_metric_id` - The UUID of the billable metric
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_billable_metric_id(mut self, billable_metric_id: Uuid) -> Self {
        self.billable_metric_id = Some(billable_metric_id);
        self
    }

    /// Sets the charge model.
    ///
    /// # Arguments
    /// * `charge_model` - The pricing model to use for this charge
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_charge_model(mut self, charge_model: ChargeModel) -> Self {
        self.charge_model = Some(charge_model);
        self
    }

    /// Sets the charge properties.
    ///
    /// # Arguments
    /// * `properties` - The configuration properties for the charge model
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_properties(mut self, properties: CreateChargeProperties) -> Self {
        self.properties = Some(properties);
        self
    }
}

impl Default for UpdateChargeInput {
    fn default() -> Self {
        Self::new()
    }
}

/// Wrapper request for updating a plan.
///
/// This struct wraps the plan update data in the format expected by the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePlanRequest {
    pub plan: UpdatePlanInput,
}

impl UpdatePlanRequest {
    /// Creates a new update plan request.
    ///
    /// # Arguments
    /// * `plan` - The plan update data
    ///
    /// # Returns
    /// A new `UpdatePlanRequest` instance
    pub fn new(plan: UpdatePlanInput) -> Self {
        Self { plan }
    }
}

/// Request parameters for deleting a plan.
///
/// Used to delete a plan by its unique code identifier. Note that plans
/// with active subscriptions typically cannot be deleted.
#[derive(Debug, Clone)]
pub struct DeletePlanRequest {
    pub code: String,
}

impl DeletePlanRequest {
    /// Creates a new delete plan request.
    ///
    /// # Arguments
    /// * `code` - The code of the plan to delete
    ///
    /// # Returns
    /// A new `DeletePlanRequest` instance
    pub fn new(code: String) -> Self {
        Self { code }
    }
}
