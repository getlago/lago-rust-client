use serde::{Deserialize, Serialize};

/// Metadata information about pagination state.
/// 
/// This struct contains information about the current page, total pages,
/// and navigation to next/previous pages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub current_page: i32,
    pub next_page: Option<i32>,
    pub prev_page: Option<i32>,
    pub total_pages: i32,
    pub total_count: i32,
}

/// A paginated response containing data and pagination metadata.
/// 
/// This is a generic wrapper for API responses that include pagination,
/// where `T` represents the type of items in the data array.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

/// Parameters for controlling pagination in API requests.
/// 
/// This struct allows specifying which page to retrieve and how many
/// items per page should be returned.
#[derive(Debug, Clone, Default)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

impl PaginationParams {
    /// Creates a new empty pagination parameters instance.
    /// 
    /// # Returns
    /// A new `PaginationParams` instance with no parameters set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the page number to retrieve.
    /// 
    /// # Arguments
    /// * `page` - The page number to retrieve (1-based)
    /// 
    /// # Returns
    /// The modified pagination parameters for method chaining.
    pub fn with_page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the number of items per page.
    /// 
    /// # Arguments
    /// * `per_page` - The number of items to return per page
    /// 
    /// # Returns
    /// The modified pagination parameters for method chaining.
    pub fn with_per_page(mut self, per_page: i32) -> Self {
        self.per_page = Some(per_page);
        self
    }

    /// Converts the pagination parameters into HTTP query parameters.
    /// 
    /// # Returns
    /// A vector of query parameter tuples containing the pagination criteria.
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params: Vec<(&'static str, String)> = Vec::new();

        if let Some(page) = self.page {
            params.push(("page", page.to_string()));
        }

        if let Some(per_page) = self.per_page {
            params.push(("per_page", per_page.to_string()));
        }

        params
    }
}