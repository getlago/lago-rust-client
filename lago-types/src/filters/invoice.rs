use serde::{Deserialize, Serialize};

use crate::filters::{
    common::ListFilters,
    customer::CustomerFilter,
    date_range::DateRangeFilter,
};

use crate::models::{
    InvoiceStatus,
    InvoicePaymentStatus,
    InvoiceType,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InvoiceFilters {
    pub customer_filter: CustomerFilter,
    pub date_filter: DateRangeFilter,
    pub status: Option<InvoiceStatus>,
    pub payment_status: Option<InvoicePaymentStatus>,
    pub invoice_type: Option<InvoiceType>,
}

impl InvoiceFilters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_customer_id(mut self, customer_id: String) -> Self {
        self.customer_filter = self.customer_filter.with_customer_id(customer_id);
        self
    }

    pub fn with_date_range(mut self, from: String, to: String) -> Self {
        self.date_filter = self.date_filter.with_date_range(from, to);
        self
    }

    pub fn with_issuing_date_from(mut self, from: String) -> Self {
        self.date_filter = self.date_filter.with_from_date(from);
        self
    }

    pub fn with_issuing_date_to(mut self, to: String) -> Self {
        self.date_filter = self.date_filter.with_to_date(to);
        self
    }

    pub fn with_status(mut self, payment_status: InvoicePaymentStatus) -> Self {
        self.payment_status = Some(payment_status);
        self
    }

    pub fn with_invoice_type(mut self, invoice_type: InvoiceType) -> Self {
        self.invoice_type = Some(invoice_type);
        self
    }
}

impl ListFilters for InvoiceFilters {
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
            params.push(("status", format!("{:?}", status).to_lowercase()));
        }
        
        if let Some(payment_status) = &self.payment_status {
            params.push(("payment_status", format!("{:?}", payment_status).to_lowercase()));
        }

        if let Some(invoice_type) = &self.invoice_type {
            params.push(("invoice_type", format!("{:?}", invoice_type).to_lowercase()));
        }

        params
    }
}