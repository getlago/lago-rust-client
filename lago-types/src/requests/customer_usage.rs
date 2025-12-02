/// Request parameters for retrieving customer current usage.
///
/// This struct contains the identifiers needed to fetch current usage data
/// for a customer's subscription within the current billing period.
#[derive(Debug, Clone)]
pub struct GetCustomerCurrentUsageRequest {
    /// The customer's external unique identifier (provided by your application)
    pub external_customer_id: String,
    /// The subscription's unique identifier within your application
    pub external_subscription_id: String,
    /// Optional flag to determine if taxes should be applied (defaults to true)
    pub apply_taxes: Option<bool>,
}

impl GetCustomerCurrentUsageRequest {
    /// Creates a new get customer current usage request.
    ///
    /// # Arguments
    /// * `external_customer_id` - The external unique identifier of the customer
    /// * `external_subscription_id` - The unique identifier of the subscription
    ///
    /// # Returns
    /// A new `GetCustomerCurrentUsageRequest` instance with the specified identifiers.
    pub fn new(external_customer_id: String, external_subscription_id: String) -> Self {
        Self {
            external_customer_id,
            external_subscription_id,
            apply_taxes: None,
        }
    }

    /// Sets the apply_taxes flag for the request.
    ///
    /// # Arguments
    /// * `apply_taxes` - Whether taxes should be applied to the usage amounts
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
    /// A vector of query parameter tuples for the subscription ID and apply_taxes flag.
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params: Vec<(&str, String)> = Vec::new();

        params.push((
            "external_subscription_id",
            self.external_subscription_id.clone(),
        ));

        if let Some(apply_taxes) = self.apply_taxes {
            params.push(("apply_taxes", apply_taxes.to_string()));
        }

        params
    }
}
