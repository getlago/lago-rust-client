use serde::{Deserialize, Serialize};

use crate::filters::common::ListFilters;
use crate::models::payment::{PayableType, PaymentProviderType, PaymentStatus, PaymentType};

/// Payment list filters. Filters combine with AND; values within each vector combine with OR.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentFilters {
    /// Match any payable payment status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_status: Option<Vec<PaymentStatus>>,
    /// Alias for payment_status; payment_status takes precedence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_statuses: Option<Vec<PaymentStatus>>,
    /// Inclusive minimum amount in integer cents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_from: Option<i64>,
    /// Inclusive maximum amount in integer cents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_to: Option<i64>,
    /// Exact, case-insensitive receipt number (at most 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    /// Inclusive start date in the organization timezone (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_from: Option<String>,
    /// Inclusive end date in the organization timezone (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_to: Option<String>,
    /// Match any payment provider type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_provider_type: Option<Vec<PaymentProviderType>>,
    /// ISO currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Exact, case-insensitive number of any invoice covered by the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,
    /// Match manual or provider payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<Vec<PaymentType>>,
    /// Match Invoice or PaymentRequest payables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payable_type: Option<Vec<PayableType>>,
    /// Search provider IDs, references, payment UUIDs, invoice numbers and customer fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_term: Option<String>,
}

impl PaymentFilters {
    /// Creates an empty set of payment filters.
    pub fn new() -> Self {
        Self::default()
    }
    /// Match any payable payment status.
    pub fn with_payment_status(mut self, value: Vec<PaymentStatus>) -> Self {
        self.payment_status = Some(value);
        self
    }
    /// Alias for payment_status; payment_status takes precedence.
    pub fn with_payment_statuses(mut self, value: Vec<PaymentStatus>) -> Self {
        self.payment_statuses = Some(value);
        self
    }
    /// Inclusive minimum amount in integer cents.
    pub fn with_amount_from(mut self, value: i64) -> Self {
        self.amount_from = Some(value);
        self
    }
    /// Inclusive maximum amount in integer cents.
    pub fn with_amount_to(mut self, value: i64) -> Self {
        self.amount_to = Some(value);
        self
    }
    /// Exact, case-insensitive receipt number (at most 255 characters).
    pub fn with_receipt_number(mut self, value: String) -> Self {
        self.receipt_number = Some(value);
        self
    }
    /// Inclusive start date in the organization timezone (YYYY-MM-DD).
    pub fn with_created_at_from(mut self, value: String) -> Self {
        self.created_at_from = Some(value);
        self
    }
    /// Inclusive end date in the organization timezone (YYYY-MM-DD).
    pub fn with_created_at_to(mut self, value: String) -> Self {
        self.created_at_to = Some(value);
        self
    }
    /// Match any payment provider type.
    pub fn with_payment_provider_type(mut self, value: Vec<PaymentProviderType>) -> Self {
        self.payment_provider_type = Some(value);
        self
    }
    /// ISO currency code.
    pub fn with_currency(mut self, value: String) -> Self {
        self.currency = Some(value);
        self
    }
    /// Exact, case-insensitive number of any invoice covered by the payment.
    pub fn with_invoice_number(mut self, value: String) -> Self {
        self.invoice_number = Some(value);
        self
    }
    /// Match manual or provider payments.
    pub fn with_payment_type(mut self, value: Vec<PaymentType>) -> Self {
        self.payment_type = Some(value);
        self
    }
    /// Match Invoice or PaymentRequest payables.
    pub fn with_payable_type(mut self, value: Vec<PayableType>) -> Self {
        self.payable_type = Some(value);
        self
    }
    /// Search provider IDs, references, payment UUIDs, invoice numbers and customer fields.
    pub fn with_search_term(mut self, value: String) -> Self {
        self.search_term = Some(value);
        self
    }
}

impl ListFilters for PaymentFilters {
    fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        for value in self.payment_status.iter().flatten() {
            params.push(("payment_status[]", format!("{value:?}").to_lowercase()));
        }
        for value in self.payment_statuses.iter().flatten() {
            params.push(("payment_statuses[]", format!("{value:?}").to_lowercase()));
        }
        if let Some(value) = &self.amount_from {
            params.push(("amount_from", value.to_string()));
        }
        if let Some(value) = &self.amount_to {
            params.push(("amount_to", value.to_string()));
        }
        if let Some(value) = &self.receipt_number {
            params.push(("receipt_number", value.to_string()));
        }
        if let Some(value) = &self.created_at_from {
            params.push(("created_at_from", value.to_string()));
        }
        if let Some(value) = &self.created_at_to {
            params.push(("created_at_to", value.to_string()));
        }
        for value in self.payment_provider_type.iter().flatten() {
            params.push((
                "payment_provider_type[]",
                format!("{value:?}").to_lowercase(),
            ));
        }
        if let Some(value) = &self.currency {
            params.push(("currency", value.to_string()));
        }
        if let Some(value) = &self.invoice_number {
            params.push(("invoice_number", value.to_string()));
        }
        for value in self.payment_type.iter().flatten() {
            params.push(("payment_type[]", format!("{value:?}").to_lowercase()));
        }
        for value in self.payable_type.iter().flatten() {
            params.push(("payable_type[]", format!("{value:?}")));
        }
        if let Some(value) = &self.search_term {
            params.push(("search_term", value.to_string()));
        }
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::PaginationParams;
    use crate::requests::payment::{ListCustomerPaymentsRequest, ListPaymentsRequest};

    #[test]
    fn filters_preserve_wire_values_and_int64_bounds() {
        let filters = PaymentFilters::new()
            .with_payment_status(vec![PaymentStatus::Succeeded, PaymentStatus::Failed])
            .with_payment_statuses(vec![PaymentStatus::Processing, PaymentStatus::Pending])
            .with_amount_from(0)
            .with_amount_to(i64::MAX)
            .with_receipt_number("Rcpt & +/#1".into())
            .with_created_at_from("2026-09-01".into())
            .with_created_at_to("2026-09-07".into())
            .with_payment_provider_type(vec![
                PaymentProviderType::Stripe,
                PaymentProviderType::Gocardless,
            ])
            .with_currency("EUR".into())
            .with_invoice_number("LAG & +/#2".into())
            .with_payment_type(vec![PaymentType::Manual, PaymentType::Provider])
            .with_payable_type(vec![PayableType::Invoice, PayableType::PaymentRequest])
            .with_search_term("pi_3 & +/#".into());
        let json = serde_json::to_value(&filters).unwrap();
        assert_eq!(json["amount_to"].as_i64(), Some(i64::MAX));
        let roundtrip: PaymentFilters = serde_json::from_value(json).unwrap();
        let expected = vec![
            ("payment_status[]", "succeeded"),
            ("payment_status[]", "failed"),
            ("payment_statuses[]", "processing"),
            ("payment_statuses[]", "pending"),
            ("amount_from", "0"),
            ("amount_to", "9223372036854775807"),
            ("receipt_number", "Rcpt & +/#1"),
            ("created_at_from", "2026-09-01"),
            ("created_at_to", "2026-09-07"),
            ("payment_provider_type[]", "stripe"),
            ("payment_provider_type[]", "gocardless"),
            ("currency", "EUR"),
            ("invoice_number", "LAG & +/#2"),
            ("payment_type[]", "manual"),
            ("payment_type[]", "provider"),
            ("payable_type[]", "Invoice"),
            ("payable_type[]", "PaymentRequest"),
            ("search_term", "pi_3 & +/#"),
        ]
        .into_iter()
        .map(|(k, v)| (k, v.to_owned()))
        .collect::<Vec<_>>();
        assert_eq!(roundtrip.to_query_params(), expected);
        let id = uuid::Uuid::parse_str("1a901a90-1a90-1a90-1a90-1a901a901a90").unwrap();
        let pagination = PaginationParams::new().with_page(2).with_per_page(5);
        let request = ListPaymentsRequest::new()
            .with_pagination(pagination.clone())
            .with_external_customer_id("cust_1".into())
            .with_invoice_id(id)
            .with_filters(filters.clone());
        let mut expected_request = vec![
            ("page", "2".into()),
            ("per_page", "5".into()),
            ("external_customer_id", "cust_1".into()),
            ("invoice_id", id.to_string()),
        ];
        expected_request.extend(expected.clone());
        assert_eq!(request.to_query_params(), expected_request);
        let customer = ListCustomerPaymentsRequest::new("cust_1".into())
            .with_pagination(pagination)
            .with_invoice_id(id)
            .with_filters(filters);
        expected_request.retain(|(k, _)| *k != "external_customer_id");
        assert_eq!(customer.to_query_params(), expected_request);
    }

    #[test]
    fn empty_filters_do_not_change_requests() {
        assert!(PaymentFilters::default().to_query_params().is_empty());
        assert!(ListPaymentsRequest::new().to_query_params().is_empty());
        assert!(
            ListCustomerPaymentsRequest::new("cust_1".into())
                .to_query_params()
                .is_empty()
        );
        assert!(
            PaymentFilters::new()
                .with_payment_status(vec![])
                .to_query_params()
                .is_empty()
        );
    }
}
