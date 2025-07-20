use serde::{Deserialize, Serialize};

/// Represents an error response from the API.
///
/// This struct contains the standard error information returned by the API
/// when a request fails, including the HTTP status code, error type, and
/// a descriptive message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub status: u16,
    pub error: String,
    pub message: String,
}
