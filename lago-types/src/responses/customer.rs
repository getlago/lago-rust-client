use serde::{Deserialize, Serialize};

use crate::models::{Customer, PaginationMeta};

/// Response for listing customers.
///
/// This struct represents the response returned when requesting a list of
/// customers, including pagination metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCustomersResponse {
    pub customers: Vec<Customer>,
    pub meta: PaginationMeta,
}

/// Response for retrieving a single customer.
///
/// This struct represents the response returned when requesting a specific
/// customer by their external ID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomerResponse {
    pub customer: Customer,
}

/// Response for creating a customer.
///
/// This struct represents the response returned when successfully creating
/// a new customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerResponse {
    pub customer: Customer,
}
