use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub current_page: i32,
    pub next_page: Option<i32>,
    pub prev_page: Option<i32>,
    pub total_pages: i32,
    pub total_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Clone, Default)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

impl PaginationParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn with_per_page(mut self, per_page: i32) -> Self {
        self.per_page = Some(per_page);
        self
    }

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