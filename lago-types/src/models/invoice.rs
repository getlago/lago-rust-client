use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::customer::Customer;
use super::usage_threshold::UsageThreshold;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub lago_id: Uuid,
    pub billing_entity_code: Option<String>,
    pub sequential_id: i32,
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
    pub version_number: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub payment_dispute_lost_at: Option<DateTime<Utc>>,
    pub payment_due_date: Option<String>,
    pub payment_overdue: Option<bool>,
    pub net_payment_term: Option<i32>,
    pub self_billed: Option<bool>,
    pub file_url: Option<String>,
    pub customer: Option<Customer>,
    pub billing_periods: Vec<InvoiceBillingPeriod>,
    pub metadata: Vec<InvoiceMetadata>,
    pub applied_taxes: Vec<InvoiceAppliedTax>,
    pub applied_usage_thresholds: Option<Vec<InvoiceAppliedUsageThreshold>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceType {
    Subscription,
    AddOn,
    Credit,
    OneOff,
    ProgressiveBilling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Draft,
    Finalized,
    Voided,
    Pending,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentStatus {
    Pending,
    Succeeded,
    Failed,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceInvoicingReason {
    SubscriptionStarting,
    SubscriptionPeriodic,
    SubscriptionTerminating,
    InAdvanceCharge,
    InAdvanceChargePeriodic,
    ProgressiveBilling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceMetadata {
    pub lago_id: Uuid,
    pub key: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceAppliedTax {
    pub lago_invoice_id: Uuid,
    pub fee_amount_cents: i64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceAppliedUsageThreshold {
    pub lifetimage_usage_amount_cents: i64,
    pub created_at: DateTime<Utc>,
    pub usage_threshold: UsageThreshold,
}