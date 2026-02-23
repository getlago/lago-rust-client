use lago_types::{
    error::{LagoError, Result},
    requests::plan_charge::{
        CreatePlanChargeFilterRequest, CreatePlanChargeRequest, CreatePlanFixedChargeRequest,
        DeletePlanChargeFilterRequest, DeletePlanChargeRequest, DeletePlanFixedChargeRequest,
        GetPlanChargeFilterRequest, GetPlanChargeRequest, GetPlanFixedChargeRequest,
        ListPlanChargeFiltersRequest, ListPlanChargesRequest, ListPlanFixedChargesRequest,
        UpdatePlanChargeFilterRequest, UpdatePlanChargeRequest, UpdatePlanFixedChargeRequest,
    },
    responses::plan_charge::{
        CreatePlanChargeFilterResponse, CreatePlanChargeResponse, CreatePlanFixedChargeResponse,
        DeletePlanChargeFilterResponse, DeletePlanChargeResponse, DeletePlanFixedChargeResponse,
        GetPlanChargeFilterResponse, GetPlanChargeResponse, GetPlanFixedChargeResponse,
        ListPlanChargeFiltersResponse, ListPlanChargesResponse, ListPlanFixedChargesResponse,
        UpdatePlanChargeFilterResponse, UpdatePlanChargeResponse, UpdatePlanFixedChargeResponse,
    },
};
use url::Url;

use crate::client::LagoClient;

impl LagoClient {
    // ─── Plan charges ────────────────────────────────────────────────────────

    pub async fn list_plan_charges(
        &self,
        request: ListPlanChargesRequest,
    ) -> Result<ListPlanChargesResponse> {
        let region = self.config.region()?;
        let mut url = Url::parse(&format!(
            "{}/plans/{}/charges",
            region.endpoint(),
            request.plan_code
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

    pub async fn get_plan_charge(
        &self,
        request: GetPlanChargeRequest,
    ) -> Result<GetPlanChargeResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!(
            "{}/plans/{}/charges/{}",
            region.endpoint(),
            request.plan_code,
            request.charge_code
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("GET", url.as_str(), None::<&()>).await
    }

    pub async fn create_plan_charge(
        &self,
        request: CreatePlanChargeRequest,
    ) -> Result<CreatePlanChargeResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!(
            "{}/plans/{}/charges",
            region.endpoint(),
            request.plan_code
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("POST", url.as_str(), Some(&request))
            .await
    }

    pub async fn update_plan_charge(
        &self,
        request: UpdatePlanChargeRequest,
    ) -> Result<UpdatePlanChargeResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!(
            "{}/plans/{}/charges/{}",
            region.endpoint(),
            request.plan_code,
            request.charge_code
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("PUT", url.as_str(), Some(&request)).await
    }

    pub async fn delete_plan_charge(
        &self,
        request: DeletePlanChargeRequest,
    ) -> Result<DeletePlanChargeResponse> {
        let region = self.config.region()?;
        let mut url = Url::parse(&format!(
            "{}/plans/{}/charges/{}",
            region.endpoint(),
            request.plan_code,
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

        self.make_request("DELETE", url.as_str(), None::<&()>).await
    }

    // ─── Plan fixed charges ──────────────────────────────────────────────────

    pub async fn list_plan_fixed_charges(
        &self,
        request: ListPlanFixedChargesRequest,
    ) -> Result<ListPlanFixedChargesResponse> {
        let region = self.config.region()?;
        let mut url = Url::parse(&format!(
            "{}/plans/{}/fixed_charges",
            region.endpoint(),
            request.plan_code
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

    pub async fn get_plan_fixed_charge(
        &self,
        request: GetPlanFixedChargeRequest,
    ) -> Result<GetPlanFixedChargeResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!(
            "{}/plans/{}/fixed_charges/{}",
            region.endpoint(),
            request.plan_code,
            request.fixed_charge_code
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("GET", url.as_str(), None::<&()>).await
    }

    pub async fn create_plan_fixed_charge(
        &self,
        request: CreatePlanFixedChargeRequest,
    ) -> Result<CreatePlanFixedChargeResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!(
            "{}/plans/{}/fixed_charges",
            region.endpoint(),
            request.plan_code
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("POST", url.as_str(), Some(&request))
            .await
    }

    pub async fn update_plan_fixed_charge(
        &self,
        request: UpdatePlanFixedChargeRequest,
    ) -> Result<UpdatePlanFixedChargeResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!(
            "{}/plans/{}/fixed_charges/{}",
            region.endpoint(),
            request.plan_code,
            request.fixed_charge_code
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("PUT", url.as_str(), Some(&request)).await
    }

    pub async fn delete_plan_fixed_charge(
        &self,
        request: DeletePlanFixedChargeRequest,
    ) -> Result<DeletePlanFixedChargeResponse> {
        let region = self.config.region()?;
        let mut url = Url::parse(&format!(
            "{}/plans/{}/fixed_charges/{}",
            region.endpoint(),
            request.plan_code,
            request.fixed_charge_code
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

    // ─── Plan charge filters ─────────────────────────────────────────────────

    pub async fn list_plan_charge_filters(
        &self,
        request: ListPlanChargeFiltersRequest,
    ) -> Result<ListPlanChargeFiltersResponse> {
        let region = self.config.region()?;
        let mut url = Url::parse(&format!(
            "{}/plans/{}/charges/{}/filters",
            region.endpoint(),
            request.plan_code,
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

    pub async fn get_plan_charge_filter(
        &self,
        request: GetPlanChargeFilterRequest,
    ) -> Result<GetPlanChargeFilterResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!(
            "{}/plans/{}/charges/{}/filters/{}",
            region.endpoint(),
            request.plan_code,
            request.charge_code,
            request.filter_id
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("GET", url.as_str(), None::<&()>).await
    }

    pub async fn create_plan_charge_filter(
        &self,
        request: CreatePlanChargeFilterRequest,
    ) -> Result<CreatePlanChargeFilterResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!(
            "{}/plans/{}/charges/{}/filters",
            region.endpoint(),
            request.plan_code,
            request.charge_code
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("POST", url.as_str(), Some(&request))
            .await
    }

    pub async fn update_plan_charge_filter(
        &self,
        request: UpdatePlanChargeFilterRequest,
    ) -> Result<UpdatePlanChargeFilterResponse> {
        let region = self.config.region()?;
        let url = Url::parse(&format!(
            "{}/plans/{}/charges/{}/filters/{}",
            region.endpoint(),
            request.plan_code,
            request.charge_code,
            request.filter_id
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        self.make_request("PUT", url.as_str(), Some(&request)).await
    }

    pub async fn delete_plan_charge_filter(
        &self,
        request: DeletePlanChargeFilterRequest,
    ) -> Result<DeletePlanChargeFilterResponse> {
        let region = self.config.region()?;
        let mut url = Url::parse(&format!(
            "{}/plans/{}/charges/{}/filters/{}",
            region.endpoint(),
            request.plan_code,
            request.charge_code,
            request.filter_id
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
