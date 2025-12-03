use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::PaginationParams;

/// Request parameters for listing payments.
///
/// This struct combines pagination parameters and optional filters
/// to build a comprehensive request for retrieving payment lists.
#[derive(Debug, Clone, Default)]
pub struct ListPaymentsRequest {
    pub pagination: PaginationParams,
    pub external_customer_id: Option<String>,
    pub invoice_id: Option<Uuid>,
}

impl ListPaymentsRequest {
    /// Creates a new empty list payments request.
    ///
    /// # Returns
    /// A new `ListPaymentsRequest` instance with default pagination and no filters.
    pub fn new() -> Self {
        Self::default()
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

    /// Sets the external customer ID filter.
    ///
    /// # Arguments
    /// * `external_customer_id` - The external customer ID to filter by
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_external_customer_id(mut self, external_customer_id: String) -> Self {
        self.external_customer_id = Some(external_customer_id);
        self
    }

    /// Sets the invoice ID filter.
    ///
    /// # Arguments
    /// * `invoice_id` - The invoice ID to filter by
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_invoice_id(mut self, invoice_id: Uuid) -> Self {
        self.invoice_id = Some(invoice_id);
        self
    }

    /// Converts the request parameters into HTTP query parameters.
    ///
    /// # Returns
    /// A vector of query parameter tuples containing both pagination and filter criteria.
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = self.pagination.to_query_params();

        if let Some(external_customer_id) = &self.external_customer_id {
            params.push(("external_customer_id", external_customer_id.clone()));
        }

        if let Some(invoice_id) = &self.invoice_id {
            params.push(("invoice_id", invoice_id.to_string()));
        }

        params
    }
}

/// Request parameters for retrieving a specific payment.
///
/// This struct contains the identifier needed to fetch a single payment
/// from the API.
#[derive(Debug, Clone)]
pub struct GetPaymentRequest {
    /// The Lago ID of the payment to retrieve.
    pub lago_id: Uuid,
}

impl GetPaymentRequest {
    /// Creates a new get payment request.
    ///
    /// # Arguments
    /// * `lago_id` - The unique Lago identifier of the payment to retrieve
    ///
    /// # Returns
    /// A new `GetPaymentRequest` instance with the specified payment ID.
    pub fn new(lago_id: Uuid) -> Self {
        Self { lago_id }
    }
}

/// Request parameters for listing a customer's payments.
///
/// This struct combines the customer identifier with pagination parameters
/// to retrieve payments for a specific customer.
#[derive(Debug, Clone)]
pub struct ListCustomerPaymentsRequest {
    /// The external customer ID.
    pub external_customer_id: String,
    /// Pagination parameters.
    pub pagination: PaginationParams,
    /// Optional invoice ID filter.
    pub invoice_id: Option<Uuid>,
}

impl ListCustomerPaymentsRequest {
    /// Creates a new list customer payments request.
    ///
    /// # Arguments
    /// * `external_customer_id` - The external customer ID
    ///
    /// # Returns
    /// A new `ListCustomerPaymentsRequest` instance.
    pub fn new(external_customer_id: String) -> Self {
        Self {
            external_customer_id,
            pagination: PaginationParams::default(),
            invoice_id: None,
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

    /// Sets the invoice ID filter.
    ///
    /// # Arguments
    /// * `invoice_id` - The invoice ID to filter by
    ///
    /// # Returns
    /// The modified request instance for method chaining.
    pub fn with_invoice_id(mut self, invoice_id: Uuid) -> Self {
        self.invoice_id = Some(invoice_id);
        self
    }

    /// Converts the request parameters into HTTP query parameters.
    ///
    /// # Returns
    /// A vector of query parameter tuples.
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = self.pagination.to_query_params();

        if let Some(invoice_id) = &self.invoice_id {
            params.push(("invoice_id", invoice_id.to_string()));
        }

        params
    }
}

/// Input for creating a payment.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatePaymentInput {
    /// Unique identifier assigned to the invoice.
    pub invoice_id: String,
    /// The payment amount in cents.
    pub amount_cents: i64,
    /// Reference for the payment.
    pub reference: String,
    /// The date the payment was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<String>,
}

impl CreatePaymentInput {
    /// Creates a new payment input.
    ///
    /// # Arguments
    /// * `invoice_id` - The invoice ID to associate with the payment
    /// * `amount_cents` - The payment amount in cents
    /// * `reference` - A reference for the payment
    ///
    /// # Returns
    /// A new `CreatePaymentInput` instance.
    pub fn new(invoice_id: String, amount_cents: i64, reference: String) -> Self {
        Self {
            invoice_id,
            amount_cents,
            reference,
            paid_at: None,
        }
    }

    /// Sets the date the payment was made.
    ///
    /// # Arguments
    /// * `paid_at` - The payment date (YYYY-MM-DD format)
    ///
    /// # Returns
    /// The modified input instance for method chaining.
    pub fn with_paid_at(mut self, paid_at: String) -> Self {
        self.paid_at = Some(paid_at);
        self
    }
}

/// Request for creating a payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentRequest {
    /// The payment input data.
    pub payment: CreatePaymentInput,
}

impl CreatePaymentRequest {
    /// Creates a new create payment request.
    ///
    /// # Arguments
    /// * `input` - The payment input data
    ///
    /// # Returns
    /// A new `CreatePaymentRequest` instance.
    pub fn new(input: CreatePaymentInput) -> Self {
        Self { payment: input }
    }
}
