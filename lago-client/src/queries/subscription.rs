use lago_types::{
    error::{LagoError, Result},
    requests::subscription::{
        CreateSubscriptionRequest, DeleteSubscriptionRequest, GetSubscriptionRequest,
        ListCustomerSubscriptionsRequest, ListSubscriptionsRequest, UpdateSubscriptionRequest,
    },
    responses::subscription::{
        CreateSubscriptionResponse, DeleteSubscriptionResponse, GetSubscriptionResponse,
        ListSubscriptionsResponse, UpdateSubscriptionResponse,
    },
};
use url::Url;

use crate::client::LagoClient;

impl LagoClient {
    /// Lists all subscriptions with optional filters and pagination.
    ///
    /// # Arguments
    /// * `request` - Optional request with pagination and filter parameters
    ///
    /// # Returns
    /// A `Result` containing the list of subscriptions or an error
    pub async fn list_subscriptions(
        &self,
        request: Option<ListSubscriptionsRequest>,
    ) -> Result<ListSubscriptionsResponse> {
        let region = self.config.region()?;
        let mut url = Url::parse(&format!("{}/subscriptions", region.endpoint()))
            .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        if let Some(req) = request {
            let query_params = req.to_query_params();

            if !query_params.is_empty() {
                let query_string = query_params
                    .iter()
                    .map(|(k, v)| format!("{k}={v}"))
                    .collect::<Vec<_>>()
                    .join("&");
                url.set_query(Some(&query_string));
            }
        }

        self.make_request("GET", url.as_str(), None::<&()>).await
    }

    /// Retrieves a specific subscription by its external ID.
    ///
    /// # Arguments
    /// * `request` - The request containing the subscription's external ID
    ///
    /// # Returns
    /// A `Result` containing the subscription or an error
    pub async fn get_subscription(
        &self,
        request: GetSubscriptionRequest,
    ) -> Result<GetSubscriptionResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!(
            "{}/subscriptions/{}",
            region.endpoint(),
            request.external_id
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("GET", url.as_str(), None::<&()>).await
    }

    /// Lists all subscriptions for a specific customer.
    ///
    /// # Arguments
    /// * `request` - The request containing the customer's external ID and optional filters
    ///
    /// # Returns
    /// A `Result` containing the list of subscriptions or an error
    pub async fn list_customer_subscriptions(
        &self,
        request: ListCustomerSubscriptionsRequest,
    ) -> Result<ListSubscriptionsResponse> {
        let region = self.config.region()?;
        let mut url = Url::parse(&format!(
            "{}/customers/{}/subscriptions",
            region.endpoint(),
            request.external_customer_id
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        let query_params = request.to_query_params();

        if !query_params.is_empty() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("&");
            url.set_query(Some(&query_string));
        }

        self.make_request("GET", url.as_str(), None::<&()>).await
    }

    /// Creates a new subscription.
    ///
    /// # Arguments
    /// * `request` - The subscription creation request with customer and plan details
    ///
    /// # Returns
    /// A `Result` containing the created subscription or an error
    pub async fn create_subscription(
        &self,
        request: CreateSubscriptionRequest,
    ) -> Result<CreateSubscriptionResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!("{}/subscriptions", region.endpoint()))
            .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("POST", url.as_str(), Some(&request))
            .await
    }

    /// Updates an existing subscription.
    ///
    /// # Arguments
    /// * `request` - The subscription update request with the external ID and update data
    ///
    /// # Returns
    /// A `Result` containing the updated subscription or an error
    pub async fn update_subscription(
        &self,
        request: UpdateSubscriptionRequest,
    ) -> Result<UpdateSubscriptionResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!(
            "{}/subscriptions/{}",
            region.endpoint(),
            request.external_id
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("PUT", url.as_str(), Some(&request)).await
    }

    /// Deletes (terminates) a subscription.
    ///
    /// # Arguments
    /// * `request` - The delete request containing the subscription's external ID
    ///
    /// # Returns
    /// A `Result` containing the terminated subscription or an error
    pub async fn delete_subscription(
        &self,
        request: DeleteSubscriptionRequest,
    ) -> Result<DeleteSubscriptionResponse> {
        let region = self.config.region()?;
        let mut url = Url::parse(&format!(
            "{}/subscriptions/{}",
            region.endpoint(),
            request.external_id
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        let query_params = request.to_query_params();

        if !query_params.is_empty() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("&");
            url.set_query(Some(&query_string));
        }

        self.make_request("DELETE", url.as_str(), None::<&()>).await
    }
}
