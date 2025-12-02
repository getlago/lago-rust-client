use serde::{Deserialize, Serialize};

use crate::models::{PaginationMeta, Subscription};

/// Response for retrieving a single subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscriptionResponse {
    pub subscription: Subscription,
}

/// Response for creating a subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionResponse {
    pub subscription: Subscription,
}

/// Response for updating a subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubscriptionResponse {
    pub subscription: Subscription,
}

/// Response for deleting a subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSubscriptionResponse {
    pub subscription: Subscription,
}

/// Response for listing subscriptions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSubscriptionsResponse {
    pub subscriptions: Vec<Subscription>,
    pub meta: PaginationMeta,
}
