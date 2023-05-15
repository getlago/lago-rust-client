/*
 * Lago API documentation
 *
 * Lago API allows your application to push customer information and metrics (events) from your application to the billing application.
 *
 * The version of the OpenAPI document: 0.32.0-beta
 * Contact: tech@getlago.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvoiceObject {
    #[serde(rename = "lago_id")]
    pub lago_id: uuid::Uuid,
    #[serde(rename = "sequential_id")]
    pub sequential_id: i32,
    #[serde(rename = "number")]
    pub number: String,
    #[serde(rename = "issuing_date")]
    pub issuing_date: String,
    #[serde(rename = "invoice_type")]
    pub invoice_type: InvoiceType,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "payment_status")]
    pub payment_status: PaymentStatus,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "fees_amount_cents")]
    pub fees_amount_cents: i32,
    #[serde(rename = "coupons_amount_cents")]
    pub coupons_amount_cents: i32,
    #[serde(rename = "credit_notes_amount_cents")]
    pub credit_notes_amount_cents: i32,
    #[serde(rename = "sub_total_vat_excluded_amount_cents")]
    pub sub_total_vat_excluded_amount_cents: i32,
    #[serde(rename = "vat_amount_cents")]
    pub vat_amount_cents: i32,
    #[serde(rename = "sub_total_vat_included_amount_cents")]
    pub sub_total_vat_included_amount_cents: i32,
    #[serde(rename = "prepaid_credit_amount_cents")]
    pub prepaid_credit_amount_cents: i32,
    #[serde(rename = "total_amount_cents")]
    pub total_amount_cents: i32,
    #[serde(rename = "version_number")]
    pub version_number: i32,
    #[serde(rename = "amount_cents")]
    pub amount_cents: i32,
    #[serde(rename = "amount_currency")]
    pub amount_currency: String,
    #[serde(rename = "vat_amount_currency")]
    pub vat_amount_currency: String,
    #[serde(rename = "credit_amount_cents")]
    pub credit_amount_cents: i32,
    #[serde(rename = "credit_amount_currency")]
    pub credit_amount_currency: String,
    #[serde(rename = "total_amount_currency")]
    pub total_amount_currency: String,
    #[serde(rename = "legacy")]
    pub legacy: bool,
    #[serde(rename = "file_url", skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    #[serde(rename = "customer")]
    pub customer: Box<crate::models::CustomerObject>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<crate::models::InvoiceMetadataObject>>,
}

impl InvoiceObject {
    pub fn new(lago_id: uuid::Uuid, sequential_id: i32, number: String, issuing_date: String, invoice_type: InvoiceType, status: Status, payment_status: PaymentStatus, currency: String, fees_amount_cents: i32, coupons_amount_cents: i32, credit_notes_amount_cents: i32, sub_total_vat_excluded_amount_cents: i32, vat_amount_cents: i32, sub_total_vat_included_amount_cents: i32, prepaid_credit_amount_cents: i32, total_amount_cents: i32, version_number: i32, amount_cents: i32, amount_currency: String, vat_amount_currency: String, credit_amount_cents: i32, credit_amount_currency: String, total_amount_currency: String, legacy: bool, customer: crate::models::CustomerObject) -> InvoiceObject {
        InvoiceObject {
            lago_id,
            sequential_id,
            number,
            issuing_date,
            invoice_type,
            status,
            payment_status,
            currency,
            fees_amount_cents,
            coupons_amount_cents,
            credit_notes_amount_cents,
            sub_total_vat_excluded_amount_cents,
            vat_amount_cents,
            sub_total_vat_included_amount_cents,
            prepaid_credit_amount_cents,
            total_amount_cents,
            version_number,
            amount_cents,
            amount_currency,
            vat_amount_currency,
            credit_amount_cents,
            credit_amount_currency,
            total_amount_currency,
            legacy,
            file_url: None,
            customer: Box::new(customer),
            metadata: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InvoiceType {
    #[serde(rename = "subscription")]
    Subscription,
    #[serde(rename = "add_on")]
    AddOn,
    #[serde(rename = "credit")]
    Credit,
}

impl Default for InvoiceType {
    fn default() -> InvoiceType {
        Self::Subscription
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "finalized")]
    Finalized,
}

impl Default for Status {
    fn default() -> Status {
        Self::Draft
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for PaymentStatus {
    fn default() -> PaymentStatus {
        Self::Pending
    }
}

