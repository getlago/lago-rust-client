use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::{Display, EnumString};
use uuid::Uuid;

/// Represents a credit note in the Lago billing system.
///
/// Credit notes are issued to refund or credit customers for invoices,
/// either partially or in full.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditNote {
    /// Unique identifier for the credit note in Lago
    pub lago_id: Uuid,
    /// Sequential identifier for the credit note
    pub sequential_id: i32,
    /// Credit note number (e.g., "LAG-1234-CN-001")
    pub number: String,
    /// Lago ID of the related invoice
    pub lago_invoice_id: Uuid,
    /// Number of the related invoice
    pub invoice_number: String,
    /// Date when the credit note was issued
    pub issuing_date: String,
    /// Status of the credit (available, consumed, voided)
    pub credit_status: Option<CreditNoteCreditStatus>,
    /// Status of the refund (pending, succeeded, failed)
    pub refund_status: Option<CreditNoteRefundStatus>,
    /// Reason for the credit note
    pub reason: CreditNoteReason,
    /// Optional description for the credit note
    pub description: Option<String>,
    /// Currency code (ISO 4217)
    pub currency: String,
    /// Total amount in cents
    pub total_amount_cents: i64,
    /// Tax amount in cents
    pub taxes_amount_cents: i64,
    /// Tax rate percentage
    pub taxes_rate: f64,
    /// Subtotal excluding taxes in cents
    pub sub_total_excluding_taxes_amount_cents: i64,
    /// Remaining balance in cents
    pub balance_amount_cents: i64,
    /// Credit amount in cents
    pub credit_amount_cents: i64,
    /// Refund amount in cents
    pub refund_amount_cents: i64,
    /// Coupon adjustment amount in cents
    pub coupons_adjustment_amount_cents: i64,
    /// URL to the generated PDF file
    pub file_url: Option<String>,
    /// Whether this is a self-billed credit note
    pub self_billed: Option<bool>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// Line items for the credit note
    pub items: Option<Vec<CreditNoteItem>>,
    /// Applied taxes
    pub applied_taxes: Option<Vec<CreditNoteAppliedTax>>,
}

/// Status of the credit on a credit note
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CreditNoteCreditStatus {
    Available,
    Consumed,
    Voided,
}

/// Status of a refund on a credit note
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CreditNoteRefundStatus {
    Pending,
    Succeeded,
    Failed,
}

/// Reason for issuing a credit note
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CreditNoteReason {
    DuplicatedCharge,
    ProductUnsatisfactory,
    OrderChange,
    OrderCancellation,
    FraudulentCharge,
    Other,
}

/// A line item in a credit note
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditNoteItem {
    /// Unique identifier for the item
    pub lago_id: Uuid,
    /// Amount in cents for this item
    pub amount_cents: i64,
    /// Currency for the amount
    pub amount_currency: String,
    /// The associated fee object
    pub fee: Option<Value>,
}

/// Tax applied to a credit note
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditNoteAppliedTax {
    /// Unique identifier for the applied tax
    pub lago_id: Option<Uuid>,
    /// Reference to the tax definition
    pub lago_tax_id: Option<Uuid>,
    /// Reference to the parent credit note
    pub lago_credit_note_id: Option<Uuid>,
    /// Name of the tax
    pub tax_name: Option<String>,
    /// Code of the tax
    pub tax_code: Option<String>,
    /// Tax rate percentage
    pub tax_rate: Option<f64>,
    /// Description of the tax
    pub tax_description: Option<String>,
    /// Tax amount in cents
    pub amount_cents: Option<i64>,
    /// Currency for the tax amount
    pub amount_currency: Option<String>,
    /// Base amount the tax was calculated on
    pub base_amount_cents: Option<i64>,
    /// Creation timestamp
    pub created_at: Option<DateTime<Utc>>,
}
