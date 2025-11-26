use serde::{Deserialize, Serialize};

use crate::models::PaginationParams;

use crate::filters::{common::ListFilters, invoice::InvoiceFilters};

/// Request parameters for listing invoices.
///
/// This struct combines pagination parameters and invoice-specific filters
/// to build a comprehensive request for retrieving invoice lists.
#[derive(Debug, Clone)]
pub struct ListInvoicesRequest {
    pub pagination: PaginationParams,
    pub filters: InvoiceFilters,
}

impl ListInvoicesRequest {
    /// Creates a new empty list invoices request.
    ///
    /// # Returns
    /// A new `ListInvoicesRequest` instance with default pagination and no filters.
    pub fn new() -> Self {
        Self {
            pagination: PaginationParams::default(),
            filters: InvoiceFilters::default(),
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

    /// Sets the invoice filters for the request.
    ///
    /// # Arguments
    /// * `filters` - The invoice filters to apply
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_filters(mut self, filters: InvoiceFilters) -> Self {
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

impl Default for ListInvoicesRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Request parameters for retrieving a specific invoice.
///
/// This struct contains the identifier needed to fetch a single invoice
/// from the API.
#[derive(Debug, Clone)]
pub struct GetInvoiceRequest {
    pub invoice_id: String,
}

impl GetInvoiceRequest {
    /// Creates a new get invoice request.
    ///
    /// # Arguments
    /// * `invoice_id` - The unique identifier of the invoice to retrieve
    ///
    /// # Returns
    /// A new `GetInvoiceRequest` instance with the specified invoice ID.
    pub fn new(invoice_id: String) -> Self {
        Self { invoice_id }
    }
}

/// Billing time determines when recurring billing cycles occur.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingTime {
    /// Billing cycle based on the specific date the subscription started (billed fully).
    Anniversary,
    /// Billing cycle at the first day of the week/month/year (billed with proration).
    Calendar,
}

/// Customer information for invoice preview.
///
/// This struct contains customer data used when generating an invoice preview.
/// It can reference an existing customer via external_id or provide inline customer details.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InvoicePreviewCustomer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_identification_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<InvoicePreviewAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_customers: Option<Vec<InvoicePreviewIntegrationCustomer>>,
}

impl InvoicePreviewCustomer {
    /// Creates a new invoice preview customer with an external ID.
    ///
    /// # Arguments
    /// * `external_id` - The unique external identifier for the customer
    ///
    /// # Returns
    /// A new `InvoicePreviewCustomer` instance
    pub fn with_external_id(external_id: String) -> Self {
        Self {
            external_id: Some(external_id),
            ..Default::default()
        }
    }

    /// Creates a new empty invoice preview customer for inline definition.
    ///
    /// # Returns
    /// A new empty `InvoicePreviewCustomer` instance
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_address(
        mut self,
        address_line1: String,
        address_line2: Option<String>,
        city: Option<String>,
        state: Option<String>,
        country: Option<String>,
    ) -> Self {
        self.address_line1 = Some(address_line1);
        self.address_line2 = address_line2;
        self.city = city;
        self.state = state;
        self.country = country;
        self
    }

    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn with_timezone(mut self, timezone: String) -> Self {
        self.timezone = Some(timezone);
        self
    }

    pub fn with_tax_identification_number(mut self, tax_id: String) -> Self {
        self.tax_identification_number = Some(tax_id);
        self
    }

    pub fn with_shipping_address(mut self, address: InvoicePreviewAddress) -> Self {
        self.shipping_address = Some(address);
        self
    }

    pub fn with_integration_customers(
        mut self,
        integrations: Vec<InvoicePreviewIntegrationCustomer>,
    ) -> Self {
        self.integration_customers = Some(integrations);
        self
    }
}

/// Address information for invoice preview.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InvoicePreviewAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zipcode: Option<String>,
}

/// Integration customer for tax providers like Anrok.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicePreviewIntegrationCustomer {
    pub integration_type: String,
    pub integration_code: String,
}

/// Coupon information for invoice preview.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicePreviewCoupon {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_type: Option<InvoicePreviewCouponType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_duration: Option<i32>,
}

impl InvoicePreviewCoupon {
    /// Creates a new coupon with the required code.
    ///
    /// # Arguments
    /// * `code` - The unique code for the coupon
    ///
    /// # Returns
    /// A new `InvoicePreviewCoupon` instance
    pub fn new(code: String) -> Self {
        Self {
            code,
            name: None,
            coupon_type: None,
            amount_cents: None,
            amount_currency: None,
            percentage_rate: None,
            frequency_duration: None,
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_fixed_amount(mut self, amount_cents: i64, currency: String) -> Self {
        self.coupon_type = Some(InvoicePreviewCouponType::FixedAmount);
        self.amount_cents = Some(amount_cents);
        self.amount_currency = Some(currency);
        self
    }

    pub fn with_percentage(mut self, percentage_rate: String) -> Self {
        self.coupon_type = Some(InvoicePreviewCouponType::Percentage);
        self.percentage_rate = Some(percentage_rate);
        self
    }

    pub fn with_frequency_duration(mut self, duration: i32) -> Self {
        self.frequency_duration = Some(duration);
        self
    }
}

/// Type of coupon discount.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePreviewCouponType {
    FixedAmount,
    Percentage,
}

/// Subscription information for invoice preview.
///
/// Used to specify existing subscriptions to include in the preview,
/// with optional plan changes or termination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicePreviewSubscriptions {
    pub external_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated_at: Option<String>,
}

impl InvoicePreviewSubscriptions {
    /// Creates a new subscriptions object with the required external IDs.
    ///
    /// # Arguments
    /// * `external_ids` - The external identifiers of the subscriptions
    ///
    /// # Returns
    /// A new `InvoicePreviewSubscriptions` instance
    pub fn new(external_ids: Vec<String>) -> Self {
        Self {
            external_ids,
            plan_code: None,
            terminated_at: None,
        }
    }

    pub fn with_plan_code(mut self, plan_code: String) -> Self {
        self.plan_code = Some(plan_code);
        self
    }

    pub fn with_terminated_at(mut self, terminated_at: String) -> Self {
        self.terminated_at = Some(terminated_at);
        self
    }
}

/// Input parameters for previewing an invoice.
///
/// This struct contains all the information needed to generate an invoice preview
/// without actually creating the invoice.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InvoicePreviewInput {
    pub customer: InvoicePreviewCustomer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_time: Option<BillingTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupons: Option<Vec<InvoicePreviewCoupon>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<InvoicePreviewSubscriptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_entity_code: Option<String>,
}

impl InvoicePreviewInput {
    /// Creates a new invoice preview input with a customer reference.
    ///
    /// # Arguments
    /// * `customer` - The customer information for the preview
    ///
    /// # Returns
    /// A new `InvoicePreviewInput` instance
    pub fn new(customer: InvoicePreviewCustomer) -> Self {
        Self {
            customer,
            ..Default::default()
        }
    }

    /// Creates an invoice preview for an existing customer.
    ///
    /// # Arguments
    /// * `external_id` - The external ID of the existing customer
    ///
    /// # Returns
    /// A new `InvoicePreviewInput` instance
    pub fn for_customer(external_id: String) -> Self {
        Self::new(InvoicePreviewCustomer::with_external_id(external_id))
    }

    pub fn with_plan_code(mut self, plan_code: String) -> Self {
        self.plan_code = Some(plan_code);
        self
    }

    pub fn with_subscription_at(mut self, subscription_at: String) -> Self {
        self.subscription_at = Some(subscription_at);
        self
    }

    pub fn with_billing_time(mut self, billing_time: BillingTime) -> Self {
        self.billing_time = Some(billing_time);
        self
    }

    pub fn with_coupons(mut self, coupons: Vec<InvoicePreviewCoupon>) -> Self {
        self.coupons = Some(coupons);
        self
    }

    pub fn with_subscriptions(mut self, subscriptions: InvoicePreviewSubscriptions) -> Self {
        self.subscriptions = Some(subscriptions);
        self
    }

    pub fn with_billing_entity_code(mut self, code: String) -> Self {
        self.billing_entity_code = Some(code);
        self
    }
}

/// Request parameters for previewing an invoice.
///
/// This struct wraps the invoice preview input in the expected API format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicePreviewRequest {
    #[serde(flatten)]
    pub input: InvoicePreviewInput,
}

impl InvoicePreviewRequest {
    /// Creates a new preview invoice request.
    ///
    /// # Arguments
    /// * `input` - The invoice preview input data
    ///
    /// # Returns
    /// A new `PreviewInvoiceRequest` instance
    pub fn new(input: InvoicePreviewInput) -> Self {
        Self { input }
    }
}
