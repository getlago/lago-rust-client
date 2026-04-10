use thiserror::Error;

/// Information about rate limit headers from the API response
#[derive(Debug, Clone)]
pub struct RateLimitInfo {
    /// Maximum number of requests allowed in the rate limit window
    pub limit: Option<u32>,
    /// Number of requests remaining in the current rate limit window
    pub remaining: Option<u32>,
    /// Number of seconds until the rate limit window resets
    pub reset: Option<u64>,
}

impl std::fmt::Display for RateLimitInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        if let Some(limit) = self.limit {
            parts.push(format!("limit={}", limit));
        }
        if let Some(remaining) = self.remaining {
            parts.push(format!("remaining={}", remaining));
        }
        if let Some(reset) = self.reset {
            parts.push(format!("reset={}s", reset));
        }
        write!(f, "{}", parts.join(", "))
    }
}

#[derive(Error, Debug)]
pub enum LagoError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },

    #[error("Invalid configuration: {0}")]
    Configuration(String),

    #[error("Unauthorized: invalid API key")]
    Unauthorized,

    #[error("Rate limit exceeded ({info})")]
    RateLimit { info: RateLimitInfo },
}

pub type Result<T> = std::result::Result<T, LagoError>;
