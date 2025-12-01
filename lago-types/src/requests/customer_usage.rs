/// Request for retrieving a customer's current usage.
///
/// This request enables the retrieval of usage-based billing data for a customer
/// within the current period.
#[derive(Debug, Clone)]
pub struct GetCustomerCurrentUsageRequest {
    /// The external unique identifier of the customer.
    pub external_customer_id: String,
    /// The unique identifier of the subscription.
    pub external_subscription_id: String,
    /// Optional flag to determine if taxes should be applied. Defaults to true.
    pub apply_taxes: Option<bool>,
}

impl GetCustomerCurrentUsageRequest {
    /// Creates a new get customer current usage request.
    ///
    /// # Arguments
    /// * `external_customer_id` - The external ID of the customer
    /// * `external_subscription_id` - The external ID of the subscription
    ///
    /// # Returns
    /// A new `GetCustomerCurrentUsageRequest` instance
    pub fn new(external_customer_id: String, external_subscription_id: String) -> Self {
        Self {
            external_customer_id,
            external_subscription_id,
            apply_taxes: None,
        }
    }

    /// Sets whether taxes should be applied.
    ///
    /// # Arguments
    /// * `apply_taxes` - Whether to apply taxes
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_apply_taxes(mut self, apply_taxes: bool) -> Self {
        self.apply_taxes = Some(apply_taxes);
        self
    }

    /// Converts the request parameters into HTTP query parameters.
    ///
    /// # Returns
    /// A vector of query parameter tuples.
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = vec![(
            "external_subscription_id",
            self.external_subscription_id.clone(),
        )];

        if let Some(apply_taxes) = self.apply_taxes {
            params.push(("apply_taxes", apply_taxes.to_string()));
        }

        params
    }
}
