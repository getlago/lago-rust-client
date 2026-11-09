use serde::{Deserialize, Serialize};

use crate::filters::{common::ListFilters, customer::CustomerFilter};
use crate::models::{
    CustomerFinalizeZeroAmountInvoice, CustomerPaymentProvider, CustomerType, PaginationParams,
    PaymentTerm,
};

#[derive(Debug, Clone)]
pub struct ListCustomersRequest {
    pub pagination: PaginationParams,
    pub filters: CustomerFilter,
}

impl ListCustomersRequest {
    /// Creates a new empty list customers request.
    ///
    /// # Returns
    /// A new `ListCustomersRequest` instance with default pagination and no filters.
    pub fn new() -> Self {
        Self {
            pagination: PaginationParams::default(),
            filters: CustomerFilter::default(),
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

    /// Sets the customer filters for the request.
    ///
    /// # Arguments
    /// * `filters` - The customer filters to apply
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_filters(mut self, filters: CustomerFilter) -> Self {
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

impl Default for ListCustomersRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct GetCustomerRequest {
    pub external_id: String,
}

impl GetCustomerRequest {
    /// Creates a new get customer request.
    ///
    /// # Arguments
    /// * `external_id` - The external ID of the customer to retrieve
    ///
    /// # Returns
    /// A new `GetCustomerRequest` instance
    pub fn new(external_id: String) -> Self {
        Self { external_id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerInput {
    pub external_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zipcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_identification_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_payment_term: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_term: Option<PaymentTerm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_type: Option<CustomerType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalize_zero_amount_invoice: Option<CustomerFinalizeZeroAmountInvoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_configuration: Option<CreateCustomerBillingConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CreateCustomerShippingAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<CreateCustomerMetadata>>,
}

impl CreateCustomerInput {
    /// Creates a new customer input with the required external ID.
    ///
    /// # Arguments
    /// * `external_id` - The unique external identifier for the customer
    ///
    /// # Returns
    /// A new `CreateCustomerInput` instance
    pub fn new(external_id: String) -> Self {
        Self {
            external_id,
            name: None,
            firstname: None,
            lastname: None,
            email: None,
            address_line1: None,
            address_line2: None,
            city: None,
            country: None,
            state: None,
            zipcode: None,
            phone: None,
            url: None,
            legal_name: None,
            legal_number: None,
            logo_url: None,
            tax_identification_number: None,
            timezone: None,
            currency: None,
            net_payment_term: None,
            payment_term: None,
            customer_type: None,
            finalize_zero_amount_invoice: None,
            billing_configuration: None,
            shipping_address: None,
            metadata: None,
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_firstname(mut self, firstname: String) -> Self {
        self.firstname = Some(firstname);
        self
    }

    pub fn with_lastname(mut self, lastname: String) -> Self {
        self.lastname = Some(lastname);
        self
    }

    pub fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn with_address(
        mut self,
        address_line1: String,
        address_line2: Option<String>,
        city: Option<String>,
        country: Option<String>,
        state: Option<String>,
        zipcode: Option<String>,
    ) -> Self {
        self.address_line1 = Some(address_line1);
        self.address_line2 = address_line2;
        self.city = city;
        self.country = country;
        self.state = state;
        self.zipcode = zipcode;
        self
    }

    pub fn with_phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
        self
    }

    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn with_legal_info(mut self, legal_name: String, legal_number: Option<String>) -> Self {
        self.legal_name = Some(legal_name);
        self.legal_number = legal_number;
        self
    }

    pub fn with_tax_identification_number(mut self, tax_id: String) -> Self {
        self.tax_identification_number = Some(tax_id);
        self
    }

    pub fn with_timezone(mut self, timezone: String) -> Self {
        self.timezone = Some(timezone);
        self
    }

    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn with_net_payment_term(mut self, days: i32) -> Self {
        self.net_payment_term = Some(days);
        self
    }

    pub fn with_payment_term(mut self, payment_term: PaymentTerm) -> Self {
        self.payment_term = Some(payment_term);
        self
    }

    pub fn with_customer_type(mut self, customer_type: CustomerType) -> Self {
        self.customer_type = Some(customer_type);
        self
    }

    pub fn with_finalize_zero_amount_invoice(
        mut self,
        setting: CustomerFinalizeZeroAmountInvoice,
    ) -> Self {
        self.finalize_zero_amount_invoice = Some(setting);
        self
    }

    pub fn with_billing_configuration(
        mut self,
        config: CreateCustomerBillingConfiguration,
    ) -> Self {
        self.billing_configuration = Some(config);
        self
    }

    pub fn with_shipping_address(mut self, address: CreateCustomerShippingAddress) -> Self {
        self.shipping_address = Some(address);
        self
    }

    pub fn with_metadata(mut self, metadata: Vec<CreateCustomerMetadata>) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerBillingConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_grace_period: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_provider: Option<CustomerPaymentProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_provider_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_with_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_payment_methods: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerShippingAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zipcode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerMetadata {
    pub key: String,
    pub value: String,
    pub display_in_invoice: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerRequest {
    pub customer: CreateCustomerInput,
}

impl CreateCustomerRequest {
    /// Creates a new create customer request.
    ///
    /// # Arguments
    /// * `customer` - The customer input data
    ///
    /// # Returns
    /// A new `CreateCustomerRequest` instance
    pub fn new(customer: CreateCustomerInput) -> Self {
        Self { customer }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::PaymentTermType;

    #[test]
    fn omits_every_unset_field() {
        let input = CreateCustomerInput::new("customer_123".to_string());
        let json = serde_json::to_value(&input).unwrap();

        assert_eq!(json, serde_json::json!({ "external_id": "customer_123" }));
    }

    #[test]
    fn omits_payment_term_when_only_net_payment_term_is_set() {
        let input = CreateCustomerInput::new("customer_123".to_string()).with_net_payment_term(30);
        let json = serde_json::to_value(&input).unwrap();

        assert_eq!(json["net_payment_term"], 30);
        assert!(json.get("payment_term").is_none());
    }

    #[test]
    fn omits_net_payment_term_when_only_payment_term_is_set() {
        let input = CreateCustomerInput::new("customer_123".to_string())
            .with_payment_term(PaymentTerm::new(PaymentTermType::EndOfMonth));
        let json = serde_json::to_value(&CreateCustomerRequest::new(input)).unwrap();

        assert_eq!(
            json["customer"]["payment_term"],
            serde_json::json!({ "term_type": "end_of_month" })
        );
        assert!(json["customer"].get("net_payment_term").is_none());
    }

    #[test]
    fn omits_unset_billing_configuration_fields() {
        let billing_configuration = CreateCustomerBillingConfiguration {
            invoice_grace_period: Some(3),
            payment_provider: None,
            payment_provider_code: None,
            provider_customer_id: None,
            sync: None,
            sync_with_provider: None,
            document_locale: None,
            provider_payment_methods: None,
        };
        let input = CreateCustomerInput::new("customer_123".to_string())
            .with_billing_configuration(billing_configuration);
        let json = serde_json::to_value(&input).unwrap();

        assert_eq!(
            json["billing_configuration"],
            serde_json::json!({ "invoice_grace_period": 3 })
        );
    }
}
