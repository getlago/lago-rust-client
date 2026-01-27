use serde::{Deserialize, Serialize};

use crate::filters::common::ListFilters;
use crate::filters::credit_note::CreditNoteFilter;
use crate::models::{CreditNoteReason, CreditNoteRefundStatus, PaginationParams};

/// Request parameters for listing credit notes.
#[derive(Debug, Clone)]
pub struct ListCreditNotesRequest {
    pub pagination: PaginationParams,
    pub filters: CreditNoteFilter,
    /// Search by id, number, customer name, external_id or email.
    pub search_term: Option<String>,
}

impl ListCreditNotesRequest {
    /// Creates a new empty list credit notes request.
    pub fn new() -> Self {
        Self {
            pagination: PaginationParams::default(),
            filters: CreditNoteFilter::default(),
            search_term: None,
        }
    }

    /// Sets the pagination parameters for the request.
    pub fn with_pagination(mut self, pagination: PaginationParams) -> Self {
        self.pagination = pagination;
        self
    }

    /// Sets the credit note filters for the request.
    pub fn with_filters(mut self, filters: CreditNoteFilter) -> Self {
        self.filters = filters;
        self
    }

    /// Sets the search term for the request.
    ///
    /// Search by id, number, customer name, external_id or email.
    pub fn with_search_term(mut self, term: String) -> Self {
        self.search_term = Some(term);
        self
    }

    /// Converts the request parameters into HTTP query parameters.
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = self.pagination.to_query_params();
        params.extend(self.filters.to_query_params());

        if let Some(ref term) = self.search_term {
            params.push(("search_term", term.clone()));
        }

        params
    }
}

impl Default for ListCreditNotesRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Request parameters for retrieving a specific credit note.
#[derive(Debug, Clone)]
pub struct GetCreditNoteRequest {
    /// The Lago ID of the credit note to retrieve
    pub lago_id: String,
}

impl GetCreditNoteRequest {
    /// Creates a new get credit note request.
    pub fn new(lago_id: String) -> Self {
        Self { lago_id }
    }
}

/// Item input for creating a credit note.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCreditNoteItemInput {
    /// The Lago ID of the fee to credit
    pub fee_id: String,
    /// The amount to credit for this fee in cents
    pub amount_cents: i64,
}

impl CreateCreditNoteItemInput {
    /// Creates a new credit note item input.
    pub fn new(fee_id: String, amount_cents: i64) -> Self {
        Self {
            fee_id,
            amount_cents,
        }
    }
}

/// Input parameters for creating a credit note.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCreditNoteInput {
    /// The Lago ID of the invoice to credit
    pub invoice_id: String,
    /// The reason for the credit note
    pub reason: CreditNoteReason,
    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The amount to be credited in cents
    pub credit_amount_cents: i64,
    /// The amount to be refunded in cents
    pub refund_amount_cents: i64,
    /// The line items for the credit note
    pub items: Vec<CreateCreditNoteItemInput>,
}

impl CreateCreditNoteInput {
    /// Creates a new credit note input.
    ///
    /// # Arguments
    /// * `invoice_id` - The Lago ID of the invoice to credit
    /// * `reason` - The reason for the credit note
    /// * `credit_amount_cents` - The amount to be credited in cents
    /// * `refund_amount_cents` - The amount to be refunded in cents
    /// * `items` - The line items for the credit note
    pub fn new(
        invoice_id: String,
        reason: CreditNoteReason,
        credit_amount_cents: i64,
        refund_amount_cents: i64,
        items: Vec<CreateCreditNoteItemInput>,
    ) -> Self {
        Self {
            invoice_id,
            reason,
            description: None,
            credit_amount_cents,
            refund_amount_cents,
            items,
        }
    }

    /// Sets the description for the credit note.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

/// Request wrapper for creating a credit note.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCreditNoteRequest {
    pub credit_note: CreateCreditNoteInput,
}

impl CreateCreditNoteRequest {
    /// Creates a new create credit note request.
    pub fn new(credit_note: CreateCreditNoteInput) -> Self {
        Self { credit_note }
    }
}

/// Input parameters for updating a credit note.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCreditNoteInput {
    /// The refund status to update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_status: Option<CreditNoteRefundStatus>,
}

impl UpdateCreditNoteInput {
    /// Creates a new empty update credit note input.
    pub fn new() -> Self {
        Self {
            refund_status: None,
        }
    }

    /// Sets the refund status.
    pub fn with_refund_status(mut self, status: CreditNoteRefundStatus) -> Self {
        self.refund_status = Some(status);
        self
    }
}

impl Default for UpdateCreditNoteInput {
    fn default() -> Self {
        Self::new()
    }
}

/// Request wrapper for updating a credit note.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCreditNoteRequest {
    /// The Lago ID of the credit note to update
    #[serde(skip)]
    pub lago_id: String,
    /// The update input
    pub credit_note: UpdateCreditNoteInput,
}

impl UpdateCreditNoteRequest {
    /// Creates a new update credit note request.
    pub fn new(lago_id: String, input: UpdateCreditNoteInput) -> Self {
        Self {
            lago_id,
            credit_note: input,
        }
    }
}
