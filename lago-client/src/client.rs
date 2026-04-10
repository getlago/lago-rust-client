use reqwest::{Client as HttpClient, Response};
use serde::de::DeserializeOwned;
use std::time::{Duration, Instant};
use tokio::time::sleep;

use lago_types::error::{LagoError, RateLimitInfo, Result};

use crate::{Config, RetryMode};

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

            match self.handle_response(response).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if !self.should_retry(&e, attempt) {
                        return Err(e);
                    }

                    attempt += 1;
                    let delay = self.get_retry_delay(&e, attempt);
                    sleep(delay).await;
                    continue;
                }
            }
        }
    }

    /// Determines the appropriate delay before the next retry attempt
    ///
    /// For rate limit errors (429), uses the `x-ratelimit-reset` header value
    /// if available, otherwise falls back to exponential backoff. For other errors,
    /// always uses exponential backoff.
    ///
    /// # Arguments
    /// * `error` - The error that occurred during the request
    /// * `attempt` - The current attempt number (1-based)
    ///
    /// # Returns
    /// The duration to wait before retrying
    fn get_retry_delay(&self, error: &LagoError, attempt: u32) -> Duration {
        match error {
            LagoError::RateLimit { info } => {
                if let Some(reset_secs) = info.reset {
                    // Use the server's reset time if available
                    Duration::from_secs(reset_secs)
                } else {
                    // Fall back to exponential backoff if header is missing
                    self.config.retry_config().delay_for_attempt(attempt)
                }
            }
            _ => {
                // Use exponential backoff for all other errors
                self.config.retry_config().delay_for_attempt(attempt)
            }
        }
    }

    /// Extracts rate limit information from response headers
    ///
    /// Parses the x-ratelimit-* headers that are present on every response
    /// from the Lago API to provide rate limit context.
    ///
    /// # Arguments
    /// * `response` - The HTTP response from the request
    ///
    /// # Returns
    /// A `RateLimitInfo` struct with parsed header values (all fields optional)
    fn parse_rate_limit_headers(&self, response: &Response) -> RateLimitInfo {
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

        RateLimitInfo {
            limit,
            remaining,
            reset,
        }
    }

    /// Processes the HTTP response and converts it to the expected type
    ///
    /// This method handles different HTTP status codes and converts them to appropriate
    /// error types for the client to handle. It also extracts rate limit information
    /// from response headers to provide context to callers.
    ///
    /// # Arguments
    /// * `response` - The HTTP response from the request
    ///
    /// # Returns
    /// A `Result` containing the deserialized response data or an error
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
            // Parse rate limit headers before consuming the response body
            let rate_limit_info = if status.as_u16() == 429 {
                Some(self.parse_rate_limit_headers(&response))
            } else {
                None
            };

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
                429 => {
                    Err(LagoError::RateLimit { info: rate_limit_info.unwrap() })
                }
                _ => Err(LagoError::Api {
                    status: status.as_u16(),
                    message: error_text,
                }),
            }
        }
    }

    /// Determines whether a request should be retried based on the error type and attempt count
    ///
    /// This method implements the retry logic based on the configured retry policy.
    /// It will retry on certain error types (HTTP errors, rate limits, server errors)
    /// but not on client errors or authentication failures.
    ///
    /// # Arguments
    /// * `error` - The error that occurred during the request
    /// * `attempt` - The current attempt number (0-based)
    ///
    /// # Returns
    /// `true` if the request should be retried, `false` otherwise
    fn should_retry(&self, error: &LagoError, attempt: u32) -> bool {
        if attempt >= self.config.retry_config().max_attempts {
            return false;
        }

        if self.config.retry_config().mode == RetryMode::Off {
            return false;
        }

        match error {
            LagoError::Http(_) => true,
            LagoError::RateLimit { .. } => true,
            LagoError::Api { status, .. } => *status >= 500,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Config, Credentials, Region, RetryConfig, RetryMode};
    use lago_types::error::{LagoError, RateLimitInfo};
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
            LagoError::RateLimit { info } => {
                assert_eq!(info.limit, Some(100));
                assert_eq!(info.remaining, Some(0));
                assert_eq!(info.reset, Some(60));
            }
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

        let rate_limit_error = LagoError::RateLimit {
            info: RateLimitInfo {
                limit: Some(100),
                remaining: Some(0),
                reset: Some(60),
            },
        };
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
    async fn test_rate_limit_error_with_headers() {
        let mut server = Server::new_async().await;
        let mock = server
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

        let result: Result<TestResponse> = client.make_request("GET", &url, None::<&()>).await;

        assert!(result.is_err());

        match result.unwrap_err() {
            LagoError::RateLimit { info } => {
                assert_eq!(info.limit, Some(100), "Expected limit to be parsed");
                assert_eq!(info.remaining, Some(0), "Expected remaining to be parsed");
                assert_eq!(info.reset, Some(120), "Expected reset seconds to be parsed");
            }
            other => panic!("Expected RateLimit error, got: {other:?}"),
        }

        mock.assert_async().await;
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
            LagoError::RateLimit { info } => {
                assert!(info.limit.is_none(), "Expected limit to be None");
                assert!(info.remaining.is_none(), "Expected remaining to be None");
                assert!(info.reset.is_none(), "Expected reset to be None");
            }
            other => panic!("Expected RateLimit error, got: {other:?}"),
        }

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_retry_delay_uses_rate_limit_reset() {
        let client = create_retry_client("http://localhost:8080", 3);

        let rate_limit_error = LagoError::RateLimit {
            info: RateLimitInfo {
                limit: Some(100),
                remaining: Some(0),
                reset: Some(45),
            },
        };

        let delay = client.get_retry_delay(&rate_limit_error, 1);
        assert_eq!(
            delay,
            Duration::from_secs(45),
            "Should use reset time from rate limit header"
        );
    }

    #[tokio::test]
    async fn test_get_retry_delay_falls_back_to_exponential_backoff() {
        let client = create_retry_client("http://localhost:8080", 3);

        let rate_limit_error = LagoError::RateLimit {
            info: RateLimitInfo {
                limit: Some(100),
                remaining: Some(0),
                reset: None,
            },
        };

        let delay = client.get_retry_delay(&rate_limit_error, 1);
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

        let delay = client.get_retry_delay(&server_error, 2);
        // With initial_delay of 100ms and multiplier of 2.0, attempt 2 should give 400ms
        assert_eq!(
            delay,
            Duration::from_millis(400),
            "Should use exponential backoff for non-rate-limit errors"
        );
    }
}
