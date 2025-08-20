use crate::models::PaginationParams;
use serde::{Deserialize, Serialize};

use crate::filters::{billable_metric::BillableMetricFilter, common::ListFilters};
use crate::models::{
    BillableMetricAggregationType, BillableMetricFilter as BillableMetricFilterModel,
    BillableMetricRoundingFunction, BillableMetricWeightedInterval,
};

/// Request parameters for listing billable metrics.
///
/// This struct combines pagination parameters and billable metric-specific filters
/// to build a comprehensive request for retrieving billable metric lists.
#[derive(Debug, Clone)]
pub struct ListBillableMetricsRequest {
    pub pagination: PaginationParams,
    pub filters: BillableMetricFilter,
}

impl ListBillableMetricsRequest {
    /// Creates a new empty list billable metrics request.
    ///
    /// # Returns
    /// A new `ListBillableMetricsRequest` instance with default pagination and no filters.
    pub fn new() -> Self {
        Self {
            pagination: PaginationParams::default(),
            filters: BillableMetricFilter::default(),
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

    /// Sets the billable metric filters for the request.
    ///
    /// # Arguments
    /// * `filters` - The billable metric filters to apply
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_filters(mut self, filters: BillableMetricFilter) -> Self {
        self.filters = filters;
        self
    }

    /// Convers the request parameters into HTTP query parameters.
    ///
    /// # Returns
    /// A vector of query parameter tuples containing both pagination and filter criteria.
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = self.pagination.to_query_params();
        params.extend(self.filters.to_query_params());
        params
    }
}

impl Default for ListBillableMetricsRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct GetBillableMetricRequest {
    pub code: String,
}

impl GetBillableMetricRequest {
    /// Creates a new get billable metric request.
    ///
    /// # Arguments
    /// * `code` - The unique code of the billable metric to retrieve
    ///
    /// # Returns
    /// A new `GetBillableMetricRequest` instance
    pub fn new(code: String) -> Self {
        Self { code }
    }
}

/// Input parameters for creating a billable metric.
///
/// This struct contains all the necessary information to create a new billable metric
/// in the Lago billing system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBillableMetricInput {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub aggregation_type: BillableMetricAggregationType,
    pub recurring: Option<bool>,
    pub rounding_function: Option<BillableMetricRoundingFunction>,
    pub rounding_precision: Option<i32>,
    pub expression: Option<String>,
    pub field_name: Option<String>,
    pub weighted_interval: Option<BillableMetricWeightedInterval>,
    pub filters: Option<Vec<BillableMetricFilterModel>>,
}

impl CreateBillableMetricInput {
    /// Creates a new billable metric input with required fields.
    ///
    /// # Arguments
    /// * `name` - The name of the billable metric
    /// * `code` - The unique code for the billable metric
    /// * `aggregation_type` - The aggregation method to use
    ///
    /// # Returns
    /// A new `CreateBillableMetricInput` instance
    pub fn new(
        name: String,
        code: String,
        aggregation_type: BillableMetricAggregationType,
    ) -> Self {
        Self {
            name,
            code,
            aggregation_type,
            description: None,
            recurring: None,
            rounding_function: None,
            rounding_precision: None,
            expression: None,
            field_name: None,
            weighted_interval: None,
            filters: None,
        }
    }

    /// Sets the description for the billable metric.
    ///
    /// # Arguments
    /// * `description` - The description of the billable metric
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets whether the billable metric is recurring.
    ///
    /// # Arguments
    /// * `recurring` - Whether the metric persists across billing periods
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_recurring(mut self, recurring: bool) -> Self {
        self.recurring = Some(recurring);
        self
    }

    /// Sets the rounding function for the billable metric.
    ///
    /// # Arguments
    /// * `rounding_function` - The rounding function to apply
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_rounding_function(
        mut self,
        rounding_function: BillableMetricRoundingFunction,
    ) -> Self {
        self.rounding_function = Some(rounding_function);
        self
    }

    /// Sets the rounding precision for the billable metric.
    ///
    /// # Arguments
    /// * `precision` - The number of decimal places for rounding
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_rounding_precision(mut self, precision: i32) -> Self {
        self.rounding_precision = Some(precision);
        self
    }

    /// Sets the expression for the billable metric.
    ///
    /// # Arguments
    /// * `expression` - The expression used to calculate event units
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_expression(mut self, expression: String) -> Self {
        self.expression = Some(expression);
        self
    }

    /// Sets the field name for the billable metric.
    ///
    /// # Arguments
    /// * `field_name` - The property to aggregate on
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_field_name(mut self, field_name: String) -> Self {
        self.field_name = Some(field_name);
        self
    }

    /// Sets the weighted interval for the billable metric.
    ///
    /// # Arguments
    /// * `interval` - The interval for weighted sum aggregation
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_weighted_interval(mut self, interval: BillableMetricWeightedInterval) -> Self {
        self.weighted_interval = Some(interval);
        self
    }

    /// Sets the filters for the billable metric.
    ///
    /// # Arguments
    /// * `filters` - The filters for differentiated pricing
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_filters(mut self, filters: Vec<BillableMetricFilterModel>) -> Self {
        self.filters = Some(filters);
        self
    }
}

/// Request parameters for creating a billable metric.
///
/// This struct wraps the billable metric input in the expected API format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBillableMetricRequest {
    pub billable_metric: CreateBillableMetricInput,
}

impl CreateBillableMetricRequest {
    /// Creates a new create billable metric request.
    ///
    /// # Arguments
    /// * `billable_metric` - The billable metric input data
    ///
    /// # Returns
    /// A new `CreateBillableMetricRequest` instance
    pub fn new(billable_metric: CreateBillableMetricInput) -> Self {
        Self { billable_metric }
    }
}
