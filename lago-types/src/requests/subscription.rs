use serde::{Deserialize, Serialize};

use crate::filters::common::ListFilters;
use crate::filters::subscription::SubscriptionFilters;
use crate::models::{PaginationParams, SubscriptionBillingTime};

/// Request parameters for listing subscriptions.
#[derive(Debug, Clone)]
pub struct ListSubscriptionsRequest {
    pub pagination: PaginationParams,
    pub filters: SubscriptionFilters,
}

impl ListSubscriptionsRequest {
    /// Creates a new empty list subscriptions request.
    pub fn new() -> Self {
        Self {
            pagination: PaginationParams::default(),
            filters: SubscriptionFilters::default(),
        }
    }

    /// Sets the pagination parameters for the request.
    pub fn with_pagination(mut self, pagination: PaginationParams) -> Self {
        self.pagination = pagination;
        self
    }

    /// Sets the subscription filters for the request.
    pub fn with_filters(mut self, filters: SubscriptionFilters) -> Self {
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

impl Default for ListSubscriptionsRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Request parameters for retrieving a specific subscription.
#[derive(Debug, Clone)]
pub struct GetSubscriptionRequest {
    /// The external unique identifier of the subscription.
    pub external_id: String,
}

impl GetSubscriptionRequest {
    /// Creates a new get subscription request.
    pub fn new(external_id: String) -> Self {
        Self { external_id }
    }
}

/// Request parameters for listing a customer's subscriptions.
#[derive(Debug, Clone)]
pub struct ListCustomerSubscriptionsRequest {
    /// The external unique identifier of the customer.
    pub external_customer_id: String,
    pub pagination: PaginationParams,
    pub filters: SubscriptionFilters,
}

impl ListCustomerSubscriptionsRequest {
    /// Creates a new list customer subscriptions request.
    pub fn new(external_customer_id: String) -> Self {
        Self {
            external_customer_id,
            pagination: PaginationParams::default(),
            filters: SubscriptionFilters::default(),
        }
    }

    /// Sets the pagination parameters for the request.
    pub fn with_pagination(mut self, pagination: PaginationParams) -> Self {
        self.pagination = pagination;
        self
    }

    /// Sets the subscription filters for the request.
    pub fn with_filters(mut self, filters: SubscriptionFilters) -> Self {
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

/// Input data for creating a subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionInput {
    /// External unique identifier for the customer.
    pub external_customer_id: String,
    /// Code of the plan to assign to the subscription.
    pub plan_code: String,
    /// Optional display name for the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional external unique identifier for the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Billing time determines when recurring billing cycles occur.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_time: Option<SubscriptionBillingTime>,
    /// The subscription start date (ISO 8601 format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_at: Option<String>,
    /// The subscription end date (ISO 8601 format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_at: Option<String>,
    /// Plan overrides to customize the plan for this subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_overrides: Option<SubscriptionPlanOverrides>,
}

impl CreateSubscriptionInput {
    /// Creates a new subscription input with required fields.
    pub fn new(external_customer_id: String, plan_code: String) -> Self {
        Self {
            external_customer_id,
            plan_code,
            name: None,
            external_id: None,
            billing_time: None,
            subscription_at: None,
            ending_at: None,
            plan_overrides: None,
        }
    }

    /// Sets the subscription name.
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the external ID.
    pub fn with_external_id(mut self, external_id: String) -> Self {
        self.external_id = Some(external_id);
        self
    }

    /// Sets the billing time.
    pub fn with_billing_time(mut self, billing_time: SubscriptionBillingTime) -> Self {
        self.billing_time = Some(billing_time);
        self
    }

    /// Sets the subscription start date.
    pub fn with_subscription_at(mut self, subscription_at: String) -> Self {
        self.subscription_at = Some(subscription_at);
        self
    }

    /// Sets the subscription end date.
    pub fn with_ending_at(mut self, ending_at: String) -> Self {
        self.ending_at = Some(ending_at);
        self
    }

    /// Sets plan overrides.
    pub fn with_plan_overrides(mut self, plan_overrides: SubscriptionPlanOverrides) -> Self {
        self.plan_overrides = Some(plan_overrides);
        self
    }
}

/// Request for creating a subscription.
#[derive(Debug, Clone, Serialize)]
pub struct CreateSubscriptionRequest {
    pub subscription: CreateSubscriptionInput,
}

impl CreateSubscriptionRequest {
    /// Creates a new create subscription request.
    pub fn new(input: CreateSubscriptionInput) -> Self {
        Self {
            subscription: input,
        }
    }
}

/// Input data for updating a subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubscriptionInput {
    /// Optional new name for the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional new end date for the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_at: Option<String>,
    /// Optional new plan code (for plan changes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_code: Option<String>,
    /// Optional new subscription date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_at: Option<String>,
    /// Plan overrides to customize the plan for this subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_overrides: Option<SubscriptionPlanOverrides>,
}

impl UpdateSubscriptionInput {
    /// Creates a new empty update subscription input.
    pub fn new() -> Self {
        Self {
            name: None,
            ending_at: None,
            plan_code: None,
            subscription_at: None,
            plan_overrides: None,
        }
    }

    /// Sets the subscription name.
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the subscription end date.
    pub fn with_ending_at(mut self, ending_at: String) -> Self {
        self.ending_at = Some(ending_at);
        self
    }

    /// Sets the plan code (for plan changes).
    pub fn with_plan_code(mut self, plan_code: String) -> Self {
        self.plan_code = Some(plan_code);
        self
    }

    /// Sets the subscription date.
    pub fn with_subscription_at(mut self, subscription_at: String) -> Self {
        self.subscription_at = Some(subscription_at);
        self
    }

    /// Sets plan overrides.
    pub fn with_plan_overrides(mut self, plan_overrides: SubscriptionPlanOverrides) -> Self {
        self.plan_overrides = Some(plan_overrides);
        self
    }
}

impl Default for UpdateSubscriptionInput {
    fn default() -> Self {
        Self::new()
    }
}

/// Request for updating a subscription.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateSubscriptionRequest {
    /// The external unique identifier of the subscription to update.
    #[serde(skip)]
    pub external_id: String,
    /// The subscription update data.
    pub subscription: UpdateSubscriptionInput,
}

impl UpdateSubscriptionRequest {
    /// Creates a new update subscription request.
    pub fn new(external_id: String, input: UpdateSubscriptionInput) -> Self {
        Self {
            external_id,
            subscription: input,
        }
    }
}

/// Request for deleting a subscription.
#[derive(Debug, Clone)]
pub struct DeleteSubscriptionRequest {
    /// The external unique identifier of the subscription to delete.
    pub external_id: String,
    /// Optional status to set the subscription to (defaults to terminated).
    pub status: Option<String>,
}

impl DeleteSubscriptionRequest {
    /// Creates a new delete subscription request.
    pub fn new(external_id: String) -> Self {
        Self {
            external_id,
            status: None,
        }
    }

    /// Sets the target status for the subscription.
    pub fn with_status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }

    /// Converts the request parameters into HTTP query parameters.
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(status) = &self.status {
            params.push(("status", status.clone()));
        }
        params
    }
}

/// Plan overrides to customize a plan for a specific subscription.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionPlanOverrides {
    /// Override the base amount in cents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i64>,
    /// Override the currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_currency: Option<String>,
    /// Override the plan description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Override the invoice display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_display_name: Option<String>,
    /// Override the plan name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Override the tax codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_codes: Option<Vec<String>>,
    /// Override the trial period in days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period: Option<f64>,
    /// Override the minimum commitment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_commitment: Option<SubscriptionMinimumCommitmentOverride>,
    /// Override specific charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges: Option<Vec<SubscriptionChargeOverride>>,
}

impl SubscriptionPlanOverrides {
    /// Creates a new empty plan overrides object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the amount in cents override.
    pub fn with_amount_cents(mut self, amount_cents: i64) -> Self {
        self.amount_cents = Some(amount_cents);
        self
    }

    /// Sets the currency override.
    pub fn with_amount_currency(mut self, currency: String) -> Self {
        self.amount_currency = Some(currency);
        self
    }

    /// Sets the description override.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the invoice display name override.
    pub fn with_invoice_display_name(mut self, name: String) -> Self {
        self.invoice_display_name = Some(name);
        self
    }

    /// Sets the name override.
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the trial period override.
    pub fn with_trial_period(mut self, days: f64) -> Self {
        self.trial_period = Some(days);
        self
    }
}

/// Minimum commitment override for a subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionMinimumCommitmentOverride {
    /// Minimum commitment amount in cents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i64>,
    /// Invoice display name for the minimum commitment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_display_name: Option<String>,
    /// Tax codes for the minimum commitment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_codes: Option<Vec<String>>,
}

/// Charge override for a subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionChargeOverride {
    /// The ID of the charge to override.
    pub id: String,
    /// Override the billable metric ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable_metric_id: Option<String>,
    /// Override the invoice display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_display_name: Option<String>,
    /// Override the minimum amount in cents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_amount_cents: Option<i64>,
    /// Override tax codes for this charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_codes: Option<Vec<String>>,
    /// Override the pricing properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
