/*
 * Lago API documentation
 *
 * Lago API allows your application to push customer information and metrics (events) from your application to the billing application.
 *
 * The version of the OpenAPI document: 0.21.0-beta
 * Contact: tech@getlago.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvoiceObject {
    #[serde(rename = "lago_id", skip_serializing_if = "Option::is_none")]
    pub lago_id: Option<String>,
    #[serde(rename = "sequential_id", skip_serializing_if = "Option::is_none")]
    pub sequential_id: Option<i32>,
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(rename = "issuing_date", skip_serializing_if = "Option::is_none")]
    pub issuing_date: Option<String>,
    #[serde(rename = "invoice_type", skip_serializing_if = "Option::is_none")]
    pub invoice_type: Option<InvoiceType>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "payment_status", skip_serializing_if = "Option::is_none")]
    pub payment_status: Option<PaymentStatus>,
    #[serde(rename = "amount_cents", skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i32>,
    #[serde(rename = "amount_currency", skip_serializing_if = "Option::is_none")]
    pub amount_currency: Option<String>,
    #[serde(rename = "vat_amount_cents", skip_serializing_if = "Option::is_none")]
    pub vat_amount_cents: Option<i32>,
    #[serde(rename = "vat_amount_currency", skip_serializing_if = "Option::is_none")]
    pub vat_amount_currency: Option<String>,
    #[serde(rename = "credit_amount_cents", skip_serializing_if = "Option::is_none")]
    pub credit_amount_cents: Option<i32>,
    #[serde(rename = "credit_amount_currency", skip_serializing_if = "Option::is_none")]
    pub credit_amount_currency: Option<String>,
    #[serde(rename = "total_amount_cents", skip_serializing_if = "Option::is_none")]
    pub total_amount_cents: Option<i32>,
    #[serde(rename = "total_amount_currency", skip_serializing_if = "Option::is_none")]
    pub total_amount_currency: Option<String>,
    #[serde(rename = "legacy", skip_serializing_if = "Option::is_none")]
    pub legacy: Option<bool>,
    #[serde(rename = "file_url", skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    #[serde(rename = "customer", skip_serializing_if = "Option::is_none")]
    pub customer: Option<Box<crate::models::CustomerObject>>,
    #[serde(rename = "subscriptions", skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<crate::models::SubscriptionObject>>,
    #[serde(rename = "fees", skip_serializing_if = "Option::is_none")]
    pub fees: Option<Vec<crate::models::FeeObject>>,
    #[serde(rename = "credits", skip_serializing_if = "Option::is_none")]
    pub credits: Option<Vec<crate::models::CreditObject>>,
}

impl InvoiceObject {
    pub fn new() -> InvoiceObject {
        InvoiceObject {
            lago_id: None,
            sequential_id: None,
            number: None,
            issuing_date: None,
            invoice_type: None,
            status: None,
            payment_status: None,
            amount_cents: None,
            amount_currency: None,
            vat_amount_cents: None,
            vat_amount_currency: None,
            credit_amount_cents: None,
            credit_amount_currency: None,
            total_amount_cents: None,
            total_amount_currency: None,
            legacy: None,
            file_url: None,
            customer: None,
            subscriptions: None,
            fees: None,
            credits: None,
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

