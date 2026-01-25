use serde::{Deserialize, Serialize};

use crate::filters::{common::ListFilters, customer::CustomerFilter, date_range::DateRangeFilter};

use crate::models::{InvoicePaymentStatus, InvoiceStatus, InvoiceType};

/// Filter parameters for invoice list operations.
///
/// This struct represents the available filters that can be applied when
/// querying invoice lists from the API, combining customer, date, status,
/// and type filters.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InvoiceFilters {
    pub customer_filter: CustomerFilter,
    pub date_filter: DateRangeFilter,
    pub status: Option<InvoiceStatus>,
    pub payment_status: Option<InvoicePaymentStatus>,
    pub invoice_type: Option<InvoiceType>,
    pub search_term: Option<String>,
}

impl InvoiceFilters {
    /// Creates a new empty invoice filter.
    ///
    /// # Returns
    /// A new `InvoiceFilters` instance with no filters set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the customer ID filter.
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

    /// Sets both the start and end dates for the issuing date filter range.
    ///
    /// # Arguments
    /// * `from` - The start date string for invoice issuing date
    /// * `to` - The end date string for invoice issuing date
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_date_range(mut self, from: String, to: String) -> Self {
        self.date_filter = self.date_filter.with_date_range(from, to);
        self
    }

    /// Sets the start date for the issuing date filter.
    ///
    /// # Arguments
    /// * `from` - The start date string for invoice issuing date
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_issuing_date_from(mut self, from: String) -> Self {
        self.date_filter = self.date_filter.with_from_date(from);
        self
    }

    /// Sets the end date for the issuing date filter.
    ///
    /// # Arguments
    /// * `to` - The end date string for invoice issuing date
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_issuing_date_to(mut self, to: String) -> Self {
        self.date_filter = self.date_filter.with_to_date(to);
        self
    }

    /// Sets the status filter.
    ///
    /// # Arguments
    /// * `status` - The invoice status to filter by
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_status(mut self, status: InvoiceStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the payment status filter
    ///
    /// # Arguments
    /// * `payment_status` - The payment status to filter by
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_payment_status(mut self, payment_status: InvoicePaymentStatus) -> Self {
        self.payment_status = Some(payment_status);
        self
    }

    /// Sets the invoice type filter.
    ///
    /// # Arguments
    /// * `invoice_type` - The invoice type to filter by
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_invoice_type(mut self, invoice_type: InvoiceType) -> Self {
        self.invoice_type = Some(invoice_type);
        self
    }

    /// Search by id, number, customer name, external_id or email.
    ///
    /// # Arguments
    /// * `term` - The search term to filter by
    ///
    /// # Returns
    /// The modified filter instance for method chaining.
    pub fn with_search_term(mut self, term: String) -> Self {
        self.search_term = Some(term);
        self
    }
}

impl ListFilters for InvoiceFilters {
    /// Converts the invoice filters into HTTP query parameters.
    ///
    /// This method combines all the individual filter criteria into a single
    /// vector of query parameters. Date filters are mapped to invoice-specific
    /// parameter names (issuing_date_from, issuing_date_to).
    ///
    /// # Returns
    /// A vector of query parameter tuples containing all the filter criteria.
    fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params: Vec<(&str, String)> = Vec::new();

        params.extend(self.customer_filter.to_query_params());

        for (key, value) in self.date_filter.to_query_params() {
            match key {
                "from_date" => params.push(("issuing_date_from", value)),
                "to_date" => params.push(("issuing_date_to", value)),
                _ => params.push((key, value)),
            }
        }

        if let Some(status) = &self.status {
            params.push(("status", format!("{status:?}").to_lowercase()));
        }

        if let Some(payment_status) = &self.payment_status {
            params.push((
                "payment_status",
                format!("{payment_status:?}").to_lowercase(),
            ));
        }

        if let Some(invoice_type) = &self.invoice_type {
            params.push(("invoice_type", format!("{invoice_type:?}").to_lowercase()));
        }

        if let Some(ref term) = self.search_term {
            params.push(("search_term", term.clone()));
        }

        params
    }
}
