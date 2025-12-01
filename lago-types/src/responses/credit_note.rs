use serde::{Deserialize, Serialize};

use crate::models::{CreditNote, PaginationMeta};

/// Response for retrieving a single credit note.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCreditNoteResponse {
    pub credit_note: CreditNote,
}

/// Response for listing credit notes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCreditNotesResponse {
    pub credit_notes: Vec<CreditNote>,
    pub meta: PaginationMeta,
}

/// Response for creating a credit note.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCreditNoteResponse {
    pub credit_note: CreditNote,
}

/// Response for updating a credit note.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCreditNoteResponse {
    pub credit_note: CreditNote,
}
