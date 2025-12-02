use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use uuid::Uuid;

use super::customer::Customer;
use super::usage_threshold::UsageThreshold;

/// Represents an invoice in the Lago billing system.
///
/// This struct contains all information about an invoice, including amounts,
/// payment status, billing periods, and associated metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub lago_id: Option<Uuid>,
    pub billing_entity_code: Option<String>,
    pub sequential_id: Option<i32>,
    pub number: String,
    pub issuing_date: String,
    pub invoice_type: InvoiceType,
    pub status: InvoiceStatus,
    pub payment_status: InvoicePaymentStatus,
    pub currency: String,
    pub fees_amount_cents: i64,
    pub coupons_amount_cents: i64,
    pub credit_notes_amount_cents: i64,
    pub sub_total_excluding_taxes_amount_cents: i64,
    pub taxes_amount_cents: i64,
    pub sub_total_including_taxes_amount_cents: i64,
    pub prepaid_credit_amount_cents: i64,
    pub progressive_billing_credit_amount_cents: i64,
    pub total_amount_cents: i64,
    pub version_number: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub payment_dispute_lost_at: Option<DateTime<Utc>>,
    pub payment_due_date: Option<String>,
    pub payment_overdue: Option<bool>,
    pub net_payment_term: Option<i32>,
    pub self_billed: Option<bool>,
    pub file_url: Option<String>,
    pub customer: Option<Customer>,
    pub billing_periods: Option<Vec<InvoiceBillingPeriod>>,
    pub metadata: Option<Vec<InvoiceMetadata>>,
    pub applied_taxes: Vec<InvoiceAppliedTax>,
    pub applied_usage_thresholds: Option<Vec<InvoiceAppliedUsageThreshold>>,
    /// Fees associated with this invoice (included when fetching a single invoice)
    pub fees: Option<Vec<Fee>>,
}

/// Represents a fee line item on an invoice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fee {
    /// Unique identifier for the fee in Lago
    pub lago_id: Uuid,
    /// Reference to the charge that generated this fee
    pub lago_charge_id: Option<Uuid>,
    /// Reference to the invoice this fee belongs to
    pub lago_invoice_id: Option<Uuid>,
    /// Reference to the subscription
    pub lago_subscription_id: Option<Uuid>,
    /// Reference to the customer
    pub lago_customer_id: Option<Uuid>,
    /// External customer ID
    pub external_customer_id: Option<String>,
    /// External subscription ID
    pub external_subscription_id: Option<String>,
    /// Fee amount in cents (excluding taxes)
    pub amount_cents: i64,
    /// Currency for the amount
    pub amount_currency: String,
    /// Precise amount as string for decimal precision
    pub precise_amount: Option<String>,
    /// Total amount including taxes in cents
    pub total_amount_cents: i64,
    /// Currency for total amount
    pub total_amount_currency: String,
    /// Precise total amount as string
    pub precise_total_amount: Option<String>,
    /// Tax amount in cents
    pub taxes_amount_cents: i64,
    /// Precise tax amount as string
    pub taxes_precise_amount: Option<String>,
    /// Tax rate percentage
    pub taxes_rate: f64,
    /// Number of units
    pub units: String,
    /// Precise unit amount as string
    pub precise_unit_amount: Option<String>,
    /// Total aggregated units
    pub total_aggregated_units: Option<String>,
    /// Number of events that contributed to this fee
    pub events_count: Option<i64>,
    /// Payment status of the fee
    pub payment_status: FeePaymentStatus,
    /// Whether this fee is paid in advance
    pub pay_in_advance: Option<bool>,
    /// Whether this fee is invoiceable
    pub invoiceable: Option<bool>,
    /// Start date of the billing period
    pub from_date: Option<String>,
    /// End date of the billing period
    pub to_date: Option<String>,
    /// When the fee was created
    pub created_at: DateTime<Utc>,
    /// When payment succeeded
    pub succeeded_at: Option<DateTime<Utc>>,
    /// When payment failed
    pub failed_at: Option<DateTime<Utc>>,
    /// When refunded
    pub refunded_at: Option<DateTime<Utc>>,
    /// Fee item details
    pub item: Option<FeeItem>,
}

/// Payment status of a fee
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FeePaymentStatus {
    Pending,
    Succeeded,
    Failed,
    Refunded,
}

/// Fee item details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeItem {
    /// Type of fee item (charge, add_on, subscription, credit, commitment)
    #[serde(rename = "type")]
    pub item_type: String,
    /// Code identifying the item
    pub code: String,
    /// Display name
    pub name: String,
    /// Description of the item
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum InvoiceType {
    Subscription,
    AddOn,
    Credit,
    OneOff,
    ProgressiveBilling,
}

/// Defines the current status of an invoice.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum InvoiceStatus {
    Draft,
    Finalized,
    Voided,
    Pending,
    Failed,
}

/// Defines the payment status of an invoice.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum InvoicePaymentStatus {
    Pending,
    Succeeded,
    Failed,
}

/// Represents a billing period associated with an invoice.
///
/// This struct contains information about the subscription and charge periods
/// that this invoice covers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceBillingPeriod {
    pub lago_subscription_id: Uuid,
    pub external_subscription_id: String,
    pub subscription_from_datetime: DateTime<Utc>,
    pub subscription_to_datetime: DateTime<Utc>,
    pub charges_from_datetime: DateTime<Utc>,
    pub charges_to_datetime: DateTime<Utc>,
    pub invoicing_reason: InvoiceInvoicingReason,
    pub lago_plan_id: Uuid,
}

/// Defines the reason for invoice generation.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum InvoiceInvoicingReason {
    SubscriptionStarting,
    SubscriptionPeriodic,
    SubscriptionTerminating,
    InAdvanceCharge,
    InAdvanceChargePeriodic,
    ProgressiveBilling,
}

/// Represents custom metadata associated with an invoice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceMetadata {
    pub lago_id: Uuid,
    pub key: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
}

/// Represents a tax applied to an invoice.
///
/// This struct contains information about taxes that have been applied
/// to the invoice, including the tax details and amounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceAppliedTax {
    pub lago_invoice_id: Uuid,
    pub fee_amount_cents: Option<i64>,
    pub lago_id: Uuid,
    pub lago_tax_id: Uuid,
    pub tax_name: String,
    pub tax_code: String,
    pub tax_rate: f32,
    pub tax_description: String,
    pub amount_cents: i64,
    pub amount_currency: String,
    pub created_at: DateTime<Utc>,
}

/// Represents a usage threshold applied to an invoice.
///
/// This struct contains information about usage thresholds that have been
/// triggered and applied to the invoice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceAppliedUsageThreshold {
    pub lifetime_usage_amount_cents: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub usage_threshold: UsageThreshold,
}
