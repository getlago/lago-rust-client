use reqwest::{Client as HttpClient, Response};
use serde::de::DeserializeOwned;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

use lago_types::error::{LagoError, Result};

use crate::{Config, RetryMode};

/// Information about rate limit headers from the API response.
///
/// Delivered to the [`RateLimitInfoCallback`] after every successful request so
/// callers can build observability around the rate limit (warn at thresholds,
/// emit metrics, etc.).
#[derive(Debug, Clone, Default)]
pub struct RateLimitInfo {
    /// Maximum number of requests allowed in the rate limit window
    pub limit: Option<u32>,
    /// Number of requests remaining in the current rate limit window
    pub remaining: Option<u32>,
    /// Number of seconds until the rate limit window resets
    pub reset: Option<u64>,
    /// HTTP method of the call (GET, POST, ...).
    pub method: String,
    /// Request URL.
    pub url: String,
}

impl RateLimitInfo {
    /// Returns the fraction of the rate limit currently used in `[0.0, 1.0]`,
    /// or `None` when the headers aren't usable (missing limit, zero limit,
    /// missing remaining).
    pub fn usage_pct(&self) -> Option<f64> {
        match (self.limit, self.remaining) {
            (Some(limit), Some(remaining)) if limit > 0 => {
                Some(1.0 - f64::from(remaining) / f64::from(limit))
            }
            _ => None,
        }
    }
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

/// Callback invoked after every successful response with parsed rate limit
/// headers.
///
/// Use this to build observability around the rate limit (warn at thresholds,
/// emit metrics, etc.). Panics raised from the callback are caught and logged
/// so they cannot break the underlying request flow.
pub type RateLimitInfoCallback = Arc<dyn Fn(&RateLimitInfo) + Send + Sync>;

/// The main client for interacting with the Lago API
///
/// This client handles HTTP requests, authentication, retries, and error handling
/// when communicating with the Lago billing API.
#[derive(Clone)]
pub struct LagoClient {
    pub(crate) config: Config,
    http_client: HttpClient,
}

impl LagoClient {
    /// Creates a new Lago client with the provided configuration
    ///
    /// # Arguments
    /// * `config` - The configuration settings for the client
    ///
    /// # Returns
    /// A new instance of `LagoClient`
    pub fn new(config: Config) -> Self {
        let http_client = HttpClient::builder()
            .timeout(config.timeout())
            .user_agent(config.user_agent())
            .build()
            .expect("Failed to create HTTP client");

        Self {
            config,
            http_client,
        }
    }

    /// Creates a new Lago client using default configuration from environment variables
    ///
    /// This method will use default settings and attempt to load credentials
    /// from environment variables.
    ///
    /// # Returns
    /// A `Result` containing a new `LagoClient` instance or an error
    pub fn from_env() -> Result<Self> {
        let config = Config::default();
        Ok(Self::new(config))
    }

    /// Makes an HTTP request to the Lago API with automatic retry logic
    ///
    /// This method handles authentication, request serialization, response deserialization,
    /// error handling, and automatic retries based on the configured retry policy.
    ///
    /// When a rate limit error (429) is encountered, the client will use the
    /// `x-ratelimit-reset` header value as the wait time if available, falling back
    /// to exponential backoff otherwise.
    ///
    /// # Arguments
    /// * `method` - The HTTP method (GET, POST, PUT, DELETE)
    /// * `url` - The full URL to make the request to
    /// * `body` - Optional request body that will be serialized as JSON
    ///
    /// # Returns
    /// A `Result` containing the deserialized response or an error
    pub(crate) async fn make_request<T, B>(
        &self,
        method: &str,
        url: &str,
        body: Option<&B>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        let credentials = self.config.credentials()?;
        let mut attempt = 0;
        loop {
            let _start_time = Instant::now();

            let mut request_builder = match method {
                "GET" => self.http_client.get(url),
                "POST" => self.http_client.post(url),
                "PUT" => self.http_client.put(url),
                "DELETE" => self.http_client.delete(url),
                _ => {
                    return Err(LagoError::Configuration(format!(
                        "Unsupported method: {method}"
                    )));
                }
            };

            request_builder = request_builder.bearer_auth(credentials.api_key());

            if let Some(body) = body {
                request_builder = request_builder.json(body);
            }

            let response = match request_builder.send().await {
                Ok(response) => response,
                Err(e) => {
                    if attempt >= self.config.retry_config().max_attempts {
                        return Err(LagoError::Http(e));
                    }

                    attempt += 1;
                    let delay = self.config.retry_config().delay_for_attempt(attempt);
                    sleep(delay).await;
                    continue;
                }
            };

            // Parse rate limit headers before consuming the response. Used
            // both to time 429 retries and to feed the on_rate_limit_info
            // callback after a successful response.
            let rate_limit_info = self.parse_rate_limit_info(&response, method, url);

            match self.handle_response(response).await {
                Ok(result) => {
                    if let Some(info) = &rate_limit_info {
                        self.emit_rate_limit_info(info);
                    }
                    return Ok(result);
                }
                Err(e) => {
                    if !self.should_retry(&e, attempt) {
                        return Err(e);
                    }

                    attempt += 1;
                    let delay = self.get_retry_delay(rate_limit_info.as_ref(), &e, attempt);
                    sleep(delay).await;
                    continue;
                }
            }
        }
    }

    /// Invokes the configured `on_rate_limit_info` callback (if any) with
    /// parsed rate limit info, catching panics so a buggy observer cannot
    /// break the request flow.
    fn emit_rate_limit_info(&self, info: &RateLimitInfo) {
        let Some(callback) = self.config.on_rate_limit_info() else {
            return;
        };

        let result = std::panic::catch_unwind(AssertUnwindSafe(|| callback(info)));
        if result.is_err() {
            eprintln!("lago: on_rate_limit_info callback panicked; suppressing");
        }
    }

    /// Determines the appropriate delay before the next retry attempt
    ///
    /// For rate limit errors (429), uses the `x-ratelimit-reset` header value
    /// if available, otherwise falls back to exponential backoff. For other errors,
    /// always uses exponential backoff.
    /// Maximum delay before a retry attempt (20 seconds).
    const MAX_RETRY_DELAY: Duration = Duration::from_secs(20);

    fn get_retry_delay(
        &self,
        rate_limit_info: Option<&RateLimitInfo>,
        error: &LagoError,
        attempt: u32,
    ) -> Duration {
        let delay = if let LagoError::RateLimit = error
            && let Some(info) = rate_limit_info
            && let Some(reset_secs) = info.reset
        {
            Duration::from_secs(reset_secs)
        } else {
            self.config.retry_config().delay_for_attempt(attempt)
        };

        delay.min(Self::MAX_RETRY_DELAY)
    }

    /// Extracts rate limit information from response headers.
    ///
    /// Returns `None` when no `x-ratelimit-*` headers are present (for
    /// example, on a self-hosted Lago instance that doesn't enforce limits)
    /// so callers can skip emission entirely when there's nothing to report.
    fn parse_rate_limit_info(
        &self,
        response: &Response,
        method: &str,
        url: &str,
    ) -> Option<RateLimitInfo> {
        let limit = response
            .headers()
            .get("x-ratelimit-limit")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse::<u32>().ok());

        let remaining = response
            .headers()
            .get("x-ratelimit-remaining")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse::<u32>().ok());

        let reset = response
            .headers()
            .get("x-ratelimit-reset")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok());

        if limit.is_none() && remaining.is_none() && reset.is_none() {
            return None;
        }

        Some(RateLimitInfo {
            limit,
            remaining,
            reset,
            method: method.to_string(),
            url: url.to_string(),
        })
    }

    /// Processes the HTTP response and converts it to the expected type
    ///
    /// This method handles different HTTP status codes and converts them to appropriate
    /// error types for the client to handle.
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            let text = response.text().await.map_err(LagoError::Http)?;
            // Handle empty responses (e.g., 200 OK with no body)
            if text.is_empty() {
                return serde_json::from_str("{}").map_err(LagoError::Serialization);
            }
            serde_json::from_str(&text).map_err(LagoError::Serialization)
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            match status.as_u16() {
                401 => Err(LagoError::Unauthorized),
                404 => Err(LagoError::Api {
                    status: status.as_u16(),
                    message: error_text,
                }),
                429 => Err(LagoError::RateLimit),
                _ => Err(LagoError::Api {
                    status: status.as_u16(),
                    message: error_text,
                }),
            }
        }
    }

    /// Determines whether a request should be retried based on the error type and attempt count
    fn should_retry(&self, error: &LagoError, attempt: u32) -> bool {
        if attempt >= self.config.retry_config().max_attempts {
            return false;
        }

        if self.config.retry_config().mode == RetryMode::Off {
            return false;
        }

        match error {
            LagoError::Http(_) => true,
            LagoError::RateLimit => true,
            LagoError::Api { status, .. } => *status >= 500,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Config, Credentials, Region, RetryConfig, RetryMode};
    use lago_types::error::LagoError;
    use mockito::Server;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::time::Duration;

    #[derive(Debug, Deserialize, Serialize)]
    struct TestResponse {
        id: String,
        name: String,
    }

    #[derive(Serialize)]
    struct TestRequest {
        name: String,
    }

    fn create_test_client(base_url: &str) -> LagoClient {
        let config = Config::builder()
            .credentials(Credentials::new("test-api-key".to_string()))
            .region(Region::Custom(base_url.to_string()))
            .timeout(Duration::from_secs(10))
            .build();

        LagoClient::new(config)
    }

    fn create_retry_client(base_url: &str, max_attempts: u32) -> LagoClient {
        let retry_config = RetryConfig::builder()
            .max_attempts(max_attempts)
            .mode(RetryMode::Adaptive)
            .build();

        let config = Config::builder()
            .credentials(Credentials::new("test-api-key".to_string()))
            .region(Region::Custom(base_url.to_string()))
            .retry_config(retry_config)
            .timeout(Duration::from_secs(5))
            .build();

        LagoClient::new(config)
    }

    #[test]
    fn test_new_client_creation() {
        let config = Config::default();
        let client = LagoClient::new(config.clone());

        assert_eq!(client.config.timeout(), config.timeout());
        assert_eq!(client.config.user_agent(), config.user_agent());
    }

    #[test]
    fn test_from_env_client_creation() {
        let result = LagoClient::from_env();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_successful_get_request() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "id": "123",
                    "name": "Test"
                })
                .to_string(),
            )
            .create_async()
            .await;

        let client = create_test_client(&server.url());
        let url = format!("{}/test", server.url());

        let result: Result<TestResponse> = client.make_request("GET", &url, None::<&()>).await;

        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.id, "123");
        assert_eq!(response.name, "Test");

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_successful_post_request() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/test")
            .with_status(201)
            .with_header("content-type", "application/json")
            .match_body(mockito::Matcher::Json(json!({
                "name": "New Item"
            })))
            .with_body(
                json!({
                    "id": "456",
                    "name": "New Item"

                })
                .to_string(),
            )
            .create_async()
            .await;

        let client = create_test_client(&server.url());
        let url = format!("{}/test", server.url());
        let request = TestRequest {
            name: "New Item".to_string(),
        };

        let result: Result<TestResponse> = client.make_request("POST", &url, Some(&request)).await;

        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.id, "456");
        assert_eq!(response.name, "New Item");

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_authentication_header() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .match_header("Authorization", "Bearer test-api-key")
            .with_status(200)
            .with_body(json!({"id": "123", "name": "Test"}).to_string())
            .create_async()
            .await;

        let client = create_test_client(&server.url());
        let url = format!("{}/test", server.url());

        let _result: Result<TestResponse> = client.make_request("GET", &url, None::<&()>).await;

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_unsupported_method() {
        let server = Server::new_async().await;
        let client = create_test_client(&server.url());
        let url = format!("{}/test", server.url());

        let result: Result<TestResponse> = client.make_request("PATCH", &url, None::<&()>).await;

        assert!(result.is_err());

        match result.unwrap_err() {
            LagoError::Configuration(msg) => {
                assert!(msg.contains("Unsupported method: PATCH"));
            }
            _ => panic!("Expected Configuration error"),
        }
    }

    #[tokio::test]
    async fn test_unauthorized_error() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(401)
            .with_body("Unauthorized")
            .create_async()
            .await;

        let client = create_test_client(&server.url());
        let url = format!("{}/test", server.url());

        let result: Result<TestResponse> = client.make_request("GET", &url, None::<&()>).await;

        assert!(result.is_err());

        match result.unwrap_err() {
            LagoError::Unauthorized => {}
            _ => panic!("Expected Unauthorized error"),
        }

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_not_found_error() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(404)
            .with_body("Not Found")
            .create_async()
            .await;

        let client = create_test_client(&server.url());
        let url = format!("{}/test", server.url());

        let result: Result<TestResponse> = client.make_request("GET", &url, None::<&()>).await;

        assert!(result.is_err());

        match result.unwrap_err() {
            LagoError::Api { status, message } => {
                assert_eq!(status, 404);
                assert_eq!(message, "Not Found");
            }
            _ => panic!("Expected Api Error"),
        }

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_rate_limit_error() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(429)
            .with_header("x-ratelimit-limit", "100")
            .with_header("x-ratelimit-remaining", "0")
            .with_header("x-ratelimit-reset", "60")
            .with_body("Rate Limited")
            .create_async()
            .await;

        let client = create_test_client(&server.url());
        let url = format!("{}/test", server.url());

        let result: Result<TestResponse> = client.make_request("GET", &url, None::<&()>).await;

        assert!(result.is_err());

        match result.unwrap_err() {
            LagoError::RateLimit => {}
            _ => panic!("Expected RateLimit error"),
        }

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_server_error() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(500)
            .with_body("Internal Server Error")
            .create_async()
            .await;

        let client = create_test_client(&server.url());
        let url = format!("{}/test", server.url());

        let result: Result<TestResponse> = client.make_request("GET", &url, None::<&()>).await;

        assert!(result.is_err());

        match result.unwrap_err() {
            LagoError::Api { status, message } => {
                assert_eq!(status, 500);
                assert_eq!(message, "Internal Server Error");
            }
            _ => panic!("Expected Api Error"),
        }

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_json_deserialization_error() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(200)
            .with_body("invalid json")
            .create_async()
            .await;

        let client = create_test_client(&server.url());
        let url = format!("{}/test", server.url());

        let result: Result<TestResponse> = client.make_request("GET", &url, None::<&()>).await;

        assert!(result.is_err());

        match result.unwrap_err() {
            LagoError::Serialization(_) => {}
            _ => panic!("Expected Serialization error"),
        }

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_retry_on_server_error() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(500)
            .with_body("Server Error")
            .expect(4)
            .create_async()
            .await;

        let client = create_retry_client(&server.url(), 3);
        let url = format!("{}/test", server.url());

        let result: Result<TestResponse> = client.make_request("GET", &url, None::<&()>).await;

        assert!(result.is_err());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_retry_then_success() {
        let mut server = Server::new_async().await;

        let mock_fail = server
            .mock("GET", "/test")
            .with_status(500)
            .with_body("Server Error")
            .expect(2)
            .create_async()
            .await;

        let mock_success = server
            .mock("GET", "/test")
            .with_status(200)
            .with_body(json!({"id": "123", "name": "Success"}).to_string())
            .expect(1)
            .create_async()
            .await;

        let client = create_retry_client(&server.url(), 5);
        let url = format!("{}/test", server.url());

        let result: Result<TestResponse> = client.make_request("GET", &url, None::<&()>).await;

        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.id, "123");
        assert_eq!(response.name, "Success");

        mock_fail.assert_async().await;
        mock_success.assert_async().await;
    }

    #[tokio::test]
    async fn test_no_retry_on_client_error() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(400)
            .with_body("Bad Request")
            .expect(1)
            .create_async()
            .await;

        let client = create_retry_client(&server.url(), 3);
        let url = format!("{}/test", server.url());

        let result: Result<TestResponse> = client.make_request("GET", &url, None::<&()>).await;

        assert!(result.is_err());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_should_retry_logic() {
        let client = create_retry_client("http://localhost:8080", 3);

        let rate_limit_error = LagoError::RateLimit;
        assert!(client.should_retry(&rate_limit_error, 1));

        let server_error = LagoError::Api {
            status: 500,
            message: "Server Error".to_string(),
        };
        assert!(client.should_retry(&server_error, 1));

        let client_error = LagoError::Api {
            status: 400,
            message: "Bad Request".to_string(),
        };
        assert!(!client.should_retry(&client_error, 1));

        assert!(!client.should_retry(&server_error, 3));

        let auth_error = LagoError::Unauthorized;
        assert!(!client.should_retry(&auth_error, 1));

        let client_no_retry = create_test_client("http://localhost:8080");
        assert!(!client_no_retry.should_retry(&server_error, 1));
    }

    #[tokio::test]
    async fn test_timeout_handling() {
        // Test timeout by using an unreachable address that will cause a timeout
        let config = Config::builder()
            .credentials(Credentials::new("test-api-key".to_string()))
            .region(Region::Custom("http://10.255.255.1:80".to_string()))
            .timeout(Duration::from_millis(100))
            .build();

        let client = LagoClient::new(config);
        let url = "http://10.255.255.1:80/test";

        let result: Result<TestResponse> = client.make_request("GET", url, None::<&()>).await;

        assert!(result.is_err());

        match result.unwrap_err() {
            LagoError::Http(_) => {}
            other => panic!("Expected HTTP error, got: {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_empty_response_body() {
        // Test that empty response bodies are handled correctly (e.g., retry_payment returns 200 with no body)
        #[derive(Debug, Default, Deserialize)]
        struct EmptyResponse {}

        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/test")
            .with_status(200)
            .with_body("")
            .create_async()
            .await;

        let client = create_test_client(&server.url());
        let url = format!("{}/test", server.url());

        let result: Result<EmptyResponse> = client.make_request("POST", &url, None::<&()>).await;

        assert!(result.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_rate_limit_headers_parsed() {
        let mut server = Server::new_async().await;
        let _mock = server
            .mock("GET", "/test")
            .with_status(429)
            .with_header("x-ratelimit-limit", "100")
            .with_header("x-ratelimit-remaining", "0")
            .with_header("x-ratelimit-reset", "120")
            .with_body("Rate Limited")
            .create_async()
            .await;

        let client = create_test_client(&server.url());
        let url = format!("{}/test", server.url());

        // Verify the error is a RateLimit error
        let result: Result<TestResponse> = client.make_request("GET", &url, None::<&()>).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            LagoError::RateLimit => {}
            other => panic!("Expected RateLimit error, got: {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_rate_limit_error_without_headers() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(429)
            .with_body("Rate Limited")
            .create_async()
            .await;

        let client = create_test_client(&server.url());
        let url = format!("{}/test", server.url());

        let result: Result<TestResponse> = client.make_request("GET", &url, None::<&()>).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            LagoError::RateLimit => {}
            other => panic!("Expected RateLimit error, got: {other:?}"),
        }

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_retry_delay_uses_rate_limit_reset() {
        let client = create_retry_client("http://localhost:8080", 3);

        let info = RateLimitInfo {
            limit: Some(100),
            remaining: Some(0),
            reset: Some(10),
            ..Default::default()
        };

        let delay = client.get_retry_delay(Some(&info), &LagoError::RateLimit, 1);
        assert_eq!(
            delay,
            Duration::from_secs(10),
            "Should use reset time from rate limit header"
        );
    }

    #[tokio::test]
    async fn test_get_retry_delay_caps_at_max() {
        let client = create_retry_client("http://localhost:8080", 3);

        let info = RateLimitInfo {
            limit: Some(100),
            remaining: Some(0),
            reset: Some(120),
            ..Default::default()
        };

        let delay = client.get_retry_delay(Some(&info), &LagoError::RateLimit, 1);
        assert_eq!(
            delay,
            Duration::from_secs(20),
            "Should cap retry delay at 20 seconds"
        );
    }

    #[tokio::test]
    async fn test_get_retry_delay_falls_back_to_exponential_backoff() {
        let client = create_retry_client("http://localhost:8080", 3);

        let info = RateLimitInfo {
            limit: Some(100),
            remaining: Some(0),
            reset: None,
            ..Default::default()
        };

        let delay = client.get_retry_delay(Some(&info), &LagoError::RateLimit, 1);
        // With initial_delay of 100ms and multiplier of 2.0, attempt 1 should give 200ms
        assert_eq!(
            delay,
            Duration::from_millis(200),
            "Should fall back to exponential backoff when reset header is missing"
        );
    }

    #[tokio::test]
    async fn test_get_retry_delay_for_non_rate_limit_errors() {
        let client = create_retry_client("http://localhost:8080", 3);

        let server_error = LagoError::Api {
            status: 500,
            message: "Server Error".to_string(),
        };

        let delay = client.get_retry_delay(None, &server_error, 2);
        // With initial_delay of 100ms and multiplier of 2.0, attempt 2 should give 400ms
        assert_eq!(
            delay,
            Duration::from_millis(400),
            "Should use exponential backoff for non-rate-limit errors"
        );
    }

    // ------------------------------------------------------------------
    // RateLimitInfo + on_rate_limit_info observability
    // ------------------------------------------------------------------

    use std::sync::Mutex;

    #[test]
    fn test_rate_limit_info_usage_pct() {
        let info = RateLimitInfo {
            limit: Some(100),
            remaining: Some(20),
            ..Default::default()
        };
        assert_eq!(info.usage_pct(), Some(0.80));

        let saturated = RateLimitInfo {
            limit: Some(100),
            remaining: Some(0),
            ..Default::default()
        };
        assert_eq!(saturated.usage_pct(), Some(1.0));
    }

    #[test]
    fn test_rate_limit_info_usage_pct_unusable() {
        // Missing limit
        assert_eq!(
            RateLimitInfo {
                limit: None,
                remaining: Some(20),
                ..Default::default()
            }
            .usage_pct(),
            None,
        );
        // Missing remaining
        assert_eq!(
            RateLimitInfo {
                limit: Some(100),
                remaining: None,
                ..Default::default()
            }
            .usage_pct(),
            None,
        );
        // Zero limit
        assert_eq!(
            RateLimitInfo {
                limit: Some(0),
                remaining: Some(0),
                ..Default::default()
            }
            .usage_pct(),
            None,
        );
    }

    fn create_observed_client(base_url: &str, callback: RateLimitInfoCallback) -> LagoClient {
        let config = Config::builder()
            .credentials(Credentials::new("test-api-key".to_string()))
            .region(Region::Custom(base_url.to_string()))
            .timeout(Duration::from_secs(10))
            .on_rate_limit_info(callback)
            .build();

        LagoClient::new(config)
    }

    #[tokio::test]
    async fn test_on_rate_limit_info_fires_on_success() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(200)
            .with_header("x-ratelimit-limit", "100")
            .with_header("x-ratelimit-remaining", "20")
            .with_header("x-ratelimit-reset", "5")
            .with_body(json!({"id": "1", "name": "ok"}).to_string())
            .create_async()
            .await;

        let captured: Arc<Mutex<Vec<RateLimitInfo>>> = Arc::new(Mutex::new(Vec::new()));
        let captured_clone = captured.clone();
        let callback: RateLimitInfoCallback = Arc::new(move |info: &RateLimitInfo| {
            captured_clone.lock().unwrap().push(info.clone());
        });

        let client = create_observed_client(&server.url(), callback);
        let url = format!("{}/test", server.url());

        let _: Result<TestResponse> = client.make_request("GET", &url, None::<&()>).await;

        // Snapshot under the lock then drop the guard before awaiting again
        // (clippy::await_holding_lock).
        let snapshot: Vec<RateLimitInfo> = {
            let guard = captured.lock().unwrap();
            guard.clone()
        };
        assert_eq!(snapshot.len(), 1);
        assert_eq!(snapshot[0].limit, Some(100));
        assert_eq!(snapshot[0].remaining, Some(20));
        assert_eq!(snapshot[0].reset, Some(5));
        assert_eq!(snapshot[0].method, "GET");
        assert_eq!(snapshot[0].usage_pct(), Some(0.80));

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_on_rate_limit_info_not_called_when_headers_absent() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(200)
            .with_body(json!({"id": "1", "name": "ok"}).to_string())
            .create_async()
            .await;

        let counter: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
        let counter_clone = counter.clone();
        let callback: RateLimitInfoCallback = Arc::new(move |_: &RateLimitInfo| {
            *counter_clone.lock().unwrap() += 1;
        });

        let client = create_observed_client(&server.url(), callback);
        let url = format!("{}/test", server.url());

        let _: Result<TestResponse> = client.make_request("GET", &url, None::<&()>).await;

        assert_eq!(*counter.lock().unwrap(), 0);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_on_rate_limit_info_panic_is_caught() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(200)
            .with_header("x-ratelimit-limit", "100")
            .with_header("x-ratelimit-remaining", "1")
            .with_header("x-ratelimit-reset", "5")
            .with_body(json!({"id": "1", "name": "ok"}).to_string())
            .create_async()
            .await;

        let callback: RateLimitInfoCallback = Arc::new(|_: &RateLimitInfo| {
            panic!("intentional");
        });

        let client = create_observed_client(&server.url(), callback);
        let url = format!("{}/test", server.url());

        let result: Result<TestResponse> = client.make_request("GET", &url, None::<&()>).await;
        assert!(result.is_ok(), "callback panic must not break the request");

        mock.assert_async().await;
    }
}
