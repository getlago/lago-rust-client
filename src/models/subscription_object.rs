/*
 * Lago API documentation
 *
 * Lago API allows your application to push customer information and metrics (events) from your application to the billing application.
 *
 * The version of the OpenAPI document: 0.20.0-beta
 * Contact: tech@getlago.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubscriptionObject {
    #[serde(rename = "lago_id", skip_serializing_if = "Option::is_none")]
    pub lago_id: Option<String>,
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "lago_customer_id", skip_serializing_if = "Option::is_none")]
    pub lago_customer_id: Option<String>,
    #[serde(rename = "external_customer_id", skip_serializing_if = "Option::is_none")]
    pub external_customer_id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "plan_code", skip_serializing_if = "Option::is_none")]
    pub plan_code: Option<String>,
    /// Subscription status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Billing time
    #[serde(rename = "billing_time", skip_serializing_if = "Option::is_none")]
    pub billing_time: Option<BillingTime>,
    #[serde(rename = "subscription_at", skip_serializing_if = "Option::is_none")]
    pub subscription_at: Option<String>,
    #[serde(rename = "started_at", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(rename = "terminated_at", skip_serializing_if = "Option::is_none")]
    pub terminated_at: Option<String>,
    #[serde(rename = "canceled_at", skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "previous_plan_code", skip_serializing_if = "Option::is_none")]
    pub previous_plan_code: Option<String>,
    #[serde(rename = "next_plan_code", skip_serializing_if = "Option::is_none")]
    pub next_plan_code: Option<String>,
    #[serde(rename = "downgrade_plan_date", skip_serializing_if = "Option::is_none")]
    pub downgrade_plan_date: Option<String>,
}

impl SubscriptionObject {
    pub fn new() -> SubscriptionObject {
        SubscriptionObject {
            lago_id: None,
            external_id: None,
            lago_customer_id: None,
            external_customer_id: None,
            name: None,
            plan_code: None,
            status: None,
            billing_time: None,
            subscription_at: None,
            started_at: None,
            terminated_at: None,
            canceled_at: None,
            created_at: None,
            previous_plan_code: None,
            next_plan_code: None,
            downgrade_plan_date: None,
        }
    }
}

/// Subscription status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "terminated")]
    Terminated,
    #[serde(rename = "canceled")]
    Canceled,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}
/// Billing time
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BillingTime {
    #[serde(rename = "calendar")]
    Calendar,
    #[serde(rename = "anniversary")]
    Anniversary,
}

impl Default for BillingTime {
    fn default() -> BillingTime {
        Self::Calendar
    }
}

