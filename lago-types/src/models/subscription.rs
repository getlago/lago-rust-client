use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use uuid::Uuid;

/// Represents a subscription in the Lago billing system.
///
/// A subscription links a customer to a plan, defining their billing cycle
/// and the charges they will be invoiced for.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    /// Unique identifier for the subscription in Lago.
    pub lago_id: Uuid,
    /// External unique identifier for the subscription.
    pub external_id: String,
    /// Lago ID of the associated customer.
    pub lago_customer_id: Uuid,
    /// External ID of the associated customer.
    pub external_customer_id: String,
    /// Determines when recurring billing cycles occur.
    pub billing_time: SubscriptionBillingTime,
    /// Optional display name for the subscription.
    pub name: Option<String>,
    /// Code of the associated plan.
    pub plan_code: String,
    /// Current status of the subscription.
    pub status: SubscriptionStatus,
    /// When the subscription was created.
    pub created_at: DateTime<Utc>,
    /// When the subscription was canceled (if applicable).
    pub canceled_at: Option<DateTime<Utc>>,
    /// When the subscription started.
    pub started_at: Option<DateTime<Utc>>,
    /// When the subscription will end.
    pub ending_at: Option<DateTime<Utc>>,
    /// The subscription date.
    pub subscription_at: DateTime<Utc>,
    /// When the subscription was terminated (if applicable).
    pub terminated_at: Option<DateTime<Utc>>,
    /// Code of the previous plan (if changed).
    pub previous_plan_code: Option<String>,
    /// Code of the upcoming plan (if scheduled for change).
    pub next_plan_code: Option<String>,
    /// Date when a downgrade will take effect.
    pub downgrade_plan_date: Option<NaiveDate>,
    /// When the trial period ended.
    pub trial_ended_at: Option<DateTime<Utc>>,
    /// Start of the current billing period.
    pub current_billing_period_started_at: Option<DateTime<Utc>>,
    /// End of the current billing period.
    pub current_billing_period_ending_at: Option<DateTime<Utc>>,
    /// The associated plan details.
    pub plan: Option<SubscriptionPlan>,
}

/// Billing time determines when recurring billing cycles occur.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SubscriptionBillingTime {
    /// Billing cycle based on the specific date the subscription started.
    Anniversary,
    /// Billing cycle at the first day of the week/month/year.
    Calendar,
}

/// Status of a subscription.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SubscriptionStatus {
    /// Subscription is active and billing.
    Active,
    /// Subscription has been canceled but not yet terminated.
    Canceled,
    /// Subscription is pending activation.
    Pending,
    /// Subscription has been terminated.
    Terminated,
}

/// Plan details associated with a subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPlan {
    /// Unique identifier for the plan in Lago.
    pub lago_id: Uuid,
    /// Name of the plan.
    pub name: String,
    /// Display name for invoices.
    pub invoice_display_name: Option<String>,
    /// When the plan was created.
    pub created_at: DateTime<Utc>,
    /// Unique code for the plan.
    pub code: String,
    /// Billing interval (weekly, monthly, quarterly, yearly).
    pub interval: String,
    /// Description of the plan.
    pub description: Option<String>,
    /// Base amount in cents.
    pub amount_cents: i64,
    /// Currency for the amount.
    pub amount_currency: String,
    /// Trial period in days.
    pub trial_period: Option<f64>,
    /// Whether the plan is billed in advance.
    pub pay_in_advance: bool,
    /// Whether charges are billed monthly for yearly plans.
    pub bill_charges_monthly: Option<bool>,
}
