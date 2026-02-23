use lago_types::{
    error::{LagoError, Result},
    requests::subscription_charge::{
        CreateSubscriptionChargeFilterRequest, DeleteSubscriptionChargeFilterRequest,
        GetSubscriptionChargeFilterRequest, GetSubscriptionChargeRequest,
        GetSubscriptionFixedChargeRequest, ListSubscriptionChargeFiltersRequest,
        ListSubscriptionChargesRequest, ListSubscriptionFixedChargesRequest,
        UpdateSubscriptionChargeFilterRequest, UpdateSubscriptionChargeRequest,
        UpdateSubscriptionFixedChargeRequest,
    },
    responses::subscription_charge::{
        CreateSubscriptionChargeFilterResponse, DeleteSubscriptionChargeFilterResponse,
        GetSubscriptionChargeFilterResponse, GetSubscriptionChargeResponse,
        GetSubscriptionFixedChargeResponse, ListSubscriptionChargeFiltersResponse,
        ListSubscriptionChargesResponse, ListSubscriptionFixedChargesResponse,
        UpdateSubscriptionChargeFilterResponse, UpdateSubscriptionChargeResponse,
        UpdateSubscriptionFixedChargeResponse,
    },
};
use url::Url;

use crate::client::LagoClient;

impl LagoClient {
    // ─── Subscription charges ────────────────────────────────────────────────

    pub async fn list_subscription_charges(
        &self,
        request: ListSubscriptionChargesRequest,
    ) -> Result<ListSubscriptionChargesResponse> {
        let region = self.config.region()?;
        let mut url = Url::parse(&format!(
            "{}/subscriptions/{}/charges",
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

        self.make_request("GET", url.as_str(), None::<&()>).await
    }

    pub async fn get_subscription_charge(
        &self,
        request: GetSubscriptionChargeRequest,
    ) -> Result<GetSubscriptionChargeResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!(
            "{}/subscriptions/{}/charges/{}",
            region.endpoint(),
            request.external_id,
            request.charge_code
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("GET", url.as_str(), None::<&()>).await
    }

    pub async fn update_subscription_charge(
        &self,
        request: UpdateSubscriptionChargeRequest,
    ) -> Result<UpdateSubscriptionChargeResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!(
            "{}/subscriptions/{}/charges/{}",
            region.endpoint(),
            request.external_id,
            request.charge_code
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("PUT", url.as_str(), Some(&request)).await
    }

    // ─── Subscription fixed charges ──────────────────────────────────────────

    pub async fn list_subscription_fixed_charges(
        &self,
        request: ListSubscriptionFixedChargesRequest,
    ) -> Result<ListSubscriptionFixedChargesResponse> {
        let region = self.config.region()?;
        let mut url = Url::parse(&format!(
            "{}/subscriptions/{}/fixed_charges",
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

        self.make_request("GET", url.as_str(), None::<&()>).await
    }

    pub async fn get_subscription_fixed_charge(
        &self,
        request: GetSubscriptionFixedChargeRequest,
    ) -> Result<GetSubscriptionFixedChargeResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!(
            "{}/subscriptions/{}/fixed_charges/{}",
            region.endpoint(),
            request.external_id,
            request.fixed_charge_code
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("GET", url.as_str(), None::<&()>).await
    }

    pub async fn update_subscription_fixed_charge(
        &self,
        request: UpdateSubscriptionFixedChargeRequest,
    ) -> Result<UpdateSubscriptionFixedChargeResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!(
            "{}/subscriptions/{}/fixed_charges/{}",
            region.endpoint(),
            request.external_id,
            request.fixed_charge_code
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("PUT", url.as_str(), Some(&request)).await
    }

    // ─── Subscription charge filters ─────────────────────────────────────────

    pub async fn list_subscription_charge_filters(
        &self,
        request: ListSubscriptionChargeFiltersRequest,
    ) -> Result<ListSubscriptionChargeFiltersResponse> {
        let region = self.config.region()?;
        let mut url = Url::parse(&format!(
            "{}/subscriptions/{}/charges/{}/filters",
            region.endpoint(),
            request.external_id,
            request.charge_code
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

    pub async fn get_subscription_charge_filter(
        &self,
        request: GetSubscriptionChargeFilterRequest,
    ) -> Result<GetSubscriptionChargeFilterResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!(
            "{}/subscriptions/{}/charges/{}/filters/{}",
            region.endpoint(),
            request.external_id,
            request.charge_code,
            request.filter_id
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("GET", url.as_str(), None::<&()>).await
    }

    pub async fn create_subscription_charge_filter(
        &self,
        request: CreateSubscriptionChargeFilterRequest,
    ) -> Result<CreateSubscriptionChargeFilterResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!(
            "{}/subscriptions/{}/charges/{}/filters",
            region.endpoint(),
            request.external_id,
            request.charge_code
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("POST", url.as_str(), Some(&request))
            .await
    }

    pub async fn update_subscription_charge_filter(
        &self,
        request: UpdateSubscriptionChargeFilterRequest,
    ) -> Result<UpdateSubscriptionChargeFilterResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!(
            "{}/subscriptions/{}/charges/{}/filters/{}",
            region.endpoint(),
            request.external_id,
            request.charge_code,
            request.filter_id
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("PUT", url.as_str(), Some(&request)).await
    }

    pub async fn delete_subscription_charge_filter(
        &self,
        request: DeleteSubscriptionChargeFilterRequest,
    ) -> Result<DeleteSubscriptionChargeFilterResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!(
            "{}/subscriptions/{}/charges/{}/filters/{}",
            region.endpoint(),
            request.external_id,
            request.charge_code,
            request.filter_id
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("DELETE", url.as_str(), None::<&()>).await
    }
}
