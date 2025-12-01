use serde::{Deserialize, Serialize};

use crate::filters::common::ListFilters;
use crate::models::{CreditNoteCreditStatus, CreditNoteReason, CreditNoteRefundStatus};

/// Filter parameters for credit note list operations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreditNoteFilter {
    pub external_customer_id: Option<String>,
    pub issuing_date_from: Option<String>,
    pub issuing_date_to: Option<String>,
    pub search_term: Option<String>,
    pub currency: Option<String>,
    pub reason: Option<CreditNoteReason>,
    pub credit_status: Option<CreditNoteCreditStatus>,
    pub refund_status: Option<CreditNoteRefundStatus>,
    pub invoice_number: Option<String>,
    pub amount_from: Option<i64>,
    pub amount_to: Option<i64>,
}

impl CreditNoteFilter {
    /// Creates a new empty credit note filter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by external customer ID.
    pub fn with_external_customer_id(mut self, customer_id: String) -> Self {
        self.external_customer_id = Some(customer_id);
        self
    }

    /// Filter by issuing date range.
    pub fn with_date_range(mut self, from: String, to: String) -> Self {
        self.issuing_date_from = Some(from);
        self.issuing_date_to = Some(to);
        self
    }

    /// Filter by issuing date from.
    pub fn with_issuing_date_from(mut self, from: String) -> Self {
        self.issuing_date_from = Some(from);
        self
    }

    /// Filter by issuing date to.
    pub fn with_issuing_date_to(mut self, to: String) -> Self {
        self.issuing_date_to = Some(to);
        self
    }

    /// Search by id, number, customer name, external_id or email.
    pub fn with_search_term(mut self, term: String) -> Self {
        self.search_term = Some(term);
        self
    }

    /// Filter by currency (ISO 4217 code).
    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    /// Filter by credit note reason.
    pub fn with_reason(mut self, reason: CreditNoteReason) -> Self {
        self.reason = Some(reason);
        self
    }

    /// Filter by credit status.
    pub fn with_credit_status(mut self, status: CreditNoteCreditStatus) -> Self {
        self.credit_status = Some(status);
        self
    }

    /// Filter by refund status.
    pub fn with_refund_status(mut self, status: CreditNoteRefundStatus) -> Self {
        self.refund_status = Some(status);
        self
    }

    /// Filter by invoice number.
    pub fn with_invoice_number(mut self, number: String) -> Self {
        self.invoice_number = Some(number);
        self
    }

    /// Filter by minimum amount in cents.
    pub fn with_amount_from(mut self, amount: i64) -> Self {
        self.amount_from = Some(amount);
        self
    }

    /// Filter by maximum amount in cents.
    pub fn with_amount_to(mut self, amount: i64) -> Self {
        self.amount_to = Some(amount);
        self
    }

    /// Filter by amount range in cents.
    pub fn with_amount_range(mut self, from: i64, to: i64) -> Self {
        self.amount_from = Some(from);
        self.amount_to = Some(to);
        self
    }
}

impl ListFilters for CreditNoteFilter {
    fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(ref customer_id) = self.external_customer_id {
            params.push(("external_customer_id", customer_id.clone()));
        }

        if let Some(ref from) = self.issuing_date_from {
            params.push(("issuing_date_from", from.clone()));
        }

        if let Some(ref to) = self.issuing_date_to {
            params.push(("issuing_date_to", to.clone()));
        }

        if let Some(ref term) = self.search_term {
            params.push(("search_term", term.clone()));
        }

        if let Some(ref currency) = self.currency {
            params.push(("currency", currency.clone()));
        }

        if let Some(ref reason) = self.reason {
            params.push(("reason", format!("{reason:?}").to_lowercase()));
        }

        if let Some(ref status) = self.credit_status {
            params.push(("credit_status", format!("{status:?}").to_lowercase()));
        }

        if let Some(ref status) = self.refund_status {
            params.push(("refund_status", format!("{status:?}").to_lowercase()));
        }

        if let Some(ref number) = self.invoice_number {
            params.push(("invoice_number", number.clone()));
        }

        if let Some(amount) = self.amount_from {
            params.push(("amount_from", amount.to_string()));
        }

        if let Some(amount) = self.amount_to {
            params.push(("amount_to", amount.to_string()));
        }

        params
    }
}
