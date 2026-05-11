use serde::{Deserialize, Serialize};

use crate::filters::{common::ListFilters, customer::CustomerFilter, date_range::DateRangeFilter};

use crate::models::{FeePaymentStatus, FeeType};

/// Filter parameters for fee list operations.
///
/// This struct represents the available filters that can be applied when
/// querying fee lists from the `/fees` API endpoint, combining customer,
/// date range, fee type, payment status, billable metric, subscription, and
/// currency filters.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FeeFilters {
    pub customer_filter: CustomerFilter,
    pub date_filter: DateRangeFilter,
    pub fee_type: Option<FeeType>,
    pub payment_status: Option<FeePaymentStatus>,
    pub billable_metric_code: Option<String>,
    pub external_subscription_id: Option<String>,
    pub currency: Option<String>,
}

impl FeeFilters {
    /// Creates a new empty fee filter.
    ///
    /// # Returns
    /// A new `FeeFilters` instance with no filters set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the external customer ID filter.
    ///
    /// # Arguments
    /// * `customer_id` - The external customer ID to filter by
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_customer_id(mut self, customer_id: String) -> Self {
        self.customer_filter = self.customer_filter.with_customer_id(customer_id);
        self
    }

    /// Sets both the start and end dates for the created-at filter range.
    ///
    /// # Arguments
    /// * `from` - The start date string (YYYY-MM-DD or ISO 8601)
    /// * `to` - The end date string (YYYY-MM-DD or ISO 8601)
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_date_range(mut self, from: String, to: String) -> Self {
        self.date_filter = self.date_filter.with_date_range(from, to);
        self
    }

    /// Sets the start date for the created-at filter.
    pub fn with_created_at_from(mut self, from: String) -> Self {
        self.date_filter = self.date_filter.with_from_date(from);
        self
    }

    /// Sets the end date for the created-at filter.
    pub fn with_created_at_to(mut self, to: String) -> Self {
        self.date_filter = self.date_filter.with_to_date(to);
        self
    }

    /// Sets the fee type filter.
    pub fn with_fee_type(mut self, fee_type: FeeType) -> Self {
        self.fee_type = Some(fee_type);
        self
    }

    /// Sets the payment status filter.
    pub fn with_payment_status(mut self, payment_status: FeePaymentStatus) -> Self {
        self.payment_status = Some(payment_status);
        self
    }

    /// Sets the billable metric code filter.
    pub fn with_billable_metric_code(mut self, code: String) -> Self {
        self.billable_metric_code = Some(code);
        self
    }

    /// Sets the external subscription ID filter.
    pub fn with_external_subscription_id(mut self, id: String) -> Self {
        self.external_subscription_id = Some(id);
        self
    }

    /// Sets the currency filter (ISO 4217 code, e.g., "USD").
    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }
}

/// Serializes a fee type enum to the snake_case wire format expected by the API.
fn fee_type_param(ft: &FeeType) -> &'static str {
    match ft {
        FeeType::Charge => "charge",
        FeeType::AddOn => "add_on",
        FeeType::Subscription => "subscription",
        FeeType::Credit => "credit",
        FeeType::Commitment => "commitment",
    }
}

/// Serializes a fee payment status enum to the snake_case wire format expected by the API.
fn fee_payment_status_param(s: &FeePaymentStatus) -> &'static str {
    match s {
        FeePaymentStatus::Pending => "pending",
        FeePaymentStatus::Succeeded => "succeeded",
        FeePaymentStatus::Failed => "failed",
        FeePaymentStatus::Refunded => "refunded",
    }
}

impl ListFilters for FeeFilters {
    /// Converts the fee filters into HTTP query parameters.
    ///
    /// Date filters are mapped to fee-specific parameter names
    /// (`created_at_from`, `created_at_to`).
    ///
    /// # Returns
    /// A vector of query parameter tuples containing all the filter criteria.
    fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params: Vec<(&str, String)> = Vec::new();

        params.extend(self.customer_filter.to_query_params());

        for (key, value) in self.date_filter.to_query_params() {
            match key {
                "from_date" => params.push(("created_at_from", value)),
                "to_date" => params.push(("created_at_to", value)),
                _ => params.push((key, value)),
            }
        }

        if let Some(fee_type) = &self.fee_type {
            params.push(("fee_type", fee_type_param(fee_type).to_string()));
        }

        if let Some(payment_status) = &self.payment_status {
            params.push((
                "payment_status",
                fee_payment_status_param(payment_status).to_string(),
            ));
        }

        if let Some(code) = &self.billable_metric_code {
            params.push(("billable_metric_code", code.clone()));
        }

        if let Some(sub_id) = &self.external_subscription_id {
            params.push(("external_subscription_id", sub_id.clone()));
        }

        if let Some(currency) = &self.currency {
            params.push(("currency", currency.clone()));
        }

        params
    }
}
