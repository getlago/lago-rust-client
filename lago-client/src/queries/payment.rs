use lago_types::{
    error::{LagoError, Result},
    requests::payment::{
        CreatePaymentRequest, GetPaymentRequest, ListCustomerPaymentsRequest, ListPaymentsRequest,
    },
    responses::payment::{CreatePaymentResponse, GetPaymentResponse, ListPaymentsResponse},
};
use url::Url;

use crate::client::LagoClient;

/// Payment-related operations for the Lago client
impl LagoClient {
    /// Retrieves a list of payments with optional filtering parameters
    ///
    /// # Arguments
    /// * `request` - Optional filtering parameters for the payment list
    ///
    /// # Returns
    /// A `Result` containing the list of payments or an error
    pub async fn list_payments(
        &self,
        request: Option<ListPaymentsRequest>,
    ) -> Result<ListPaymentsResponse> {
        let request = request.unwrap_or_default();
        let region = self.config.region()?;
        let mut url = Url::parse(&format!("{}/payments", region.endpoint()))
            .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        let query_params = request.to_query_params();

        if !query_params.is_empty() {
            url.query_pairs_mut().extend_pairs(query_params);
        }

        self.make_request("GET", url.as_str(), None::<&()>).await
    }

    /// Retrieves a specific payment by its Lago ID
    ///
    /// # Arguments
    /// * `request` - The request containing the payment ID to retrieve
    ///
    /// # Returns
    /// A `Result` containing the payment data or an error
    pub async fn get_payment(&self, request: GetPaymentRequest) -> Result<GetPaymentResponse> {
        let region = self.config.region()?;
        let url = format!("{}/payments/{}", region.endpoint(), request.lago_id);
        self.make_request("GET", &url, None::<&()>).await
    }

    /// Creates a manual payment for an invoice
    ///
    /// # Arguments
    /// * `request` - The request containing the payment details
    ///
    /// # Returns
    /// A `Result` containing the created payment or an error
    pub async fn create_payment(
        &self,
        request: CreatePaymentRequest,
    ) -> Result<CreatePaymentResponse> {
        let region = self.config.region()?;
        let url = format!("{}/payments", region.endpoint());
        self.make_request("POST", &url, Some(&request)).await
    }

    /// Retrieves a list of payments for a specific customer
    ///
    /// # Arguments
    /// * `request` - The request containing the customer ID and optional filters
    ///
    /// # Returns
    /// A `Result` containing the list of payments or an error
    pub async fn list_customer_payments(
        &self,
        request: ListCustomerPaymentsRequest,
    ) -> Result<ListPaymentsResponse> {
        let region = self.config.region()?;
        let mut url = Url::parse(&format!(
            "{}/customers/{}/payments",
            region.endpoint(),
            urlencoding::encode(&request.external_customer_id)
        ))
        .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        let query_params = request.to_query_params();

        if !query_params.is_empty() {
            url.query_pairs_mut().extend_pairs(query_params);
        }

        self.make_request("GET", url.as_str(), None::<&()>).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Config, Credentials, Region};
    use lago_types::filters::payment::{PaymentFilters, PaymentMethodType};
    use lago_types::models::payment::PaymentStatus;
    use mockito::{Matcher, Server};

    #[tokio::test]
    async fn payment_list_urls_encode_arrays_and_exact_filter_text() {
        for customer_scoped in [false, true] {
            let mut server = Server::new_async().await;
            let client = LagoClient::new(
                Config::builder()
                    .credentials(Credentials::new("test-key"))
                    .region(Region::Custom(server.url()))
                    .build(),
            );
            let path = if customer_scoped {
                "/customers/cust_1/payments"
            } else {
                "/payments"
            };
            let mock = server.mock("GET", path)
                .match_header("Authorization", "Bearer test-key")
                .match_query(Matcher::AllOf(vec![
                    Matcher::Regex("(?:^|&)payment_status%5B%5D=succeeded(?:&|$)".into()),
                    Matcher::Regex("(?:^|&)payment_status%5B%5D=failed(?:&|$)".into()),
                    Matcher::Regex("(?:^|&)payment_method_type%5B%5D=card(?:&|$)".into()),
                    Matcher::Regex("(?:^|&)payment_method_type%5B%5D=sepa_debit(?:&|$)".into()),
                    Matcher::UrlEncoded("amount_from".into(), "0".into()),
                    Matcher::UrlEncoded("amount_to".into(), "9223372036854775807".into()),
                    Matcher::UrlEncoded("receipt_number".into(), "Rcpt & +/#1".into()),
                    Matcher::UrlEncoded("invoice_number".into(), "LAG & +/#2".into()),
                    Matcher::UrlEncoded("search_term".into(), "pi_3 & +/#".into()),
                ]))
                .with_status(200).with_header("content-type", "application/json")
                .with_body(r#"{"payments":[],"meta":{"current_page":1,"next_page":null,"prev_page":null,"total_pages":0,"total_count":0}}"#)
                .create_async().await;
            let filters = PaymentFilters::new()
                .with_payment_status(vec![PaymentStatus::Succeeded, PaymentStatus::Failed])
                .with_payment_method_type(vec![
                    PaymentMethodType::Card,
                    PaymentMethodType::SepaDebit,
                ])
                .with_amount_from(0)
                .with_amount_to(i64::MAX)
                .with_receipt_number("Rcpt & +/#1".into())
                .with_invoice_number("LAG & +/#2".into())
                .with_search_term("pi_3 & +/#".into());
            let response = if customer_scoped {
                client
                    .list_customer_payments(
                        ListCustomerPaymentsRequest::new("cust_1".into()).with_filters(filters),
                    )
                    .await
            } else {
                client
                    .list_payments(Some(ListPaymentsRequest::new().with_filters(filters)))
                    .await
            }
            .unwrap();
            assert_eq!(response.meta.total_count, 0);
            mock.assert_async().await;
        }
    }
}
