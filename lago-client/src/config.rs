use std::sync::Arc;
use std::time::Duration;

use super::{
    credentials::{
        Credentials, CredentialsProvider, EnvironmentCredentialsProvider, StaticCredentialsProvider,
    },
    region::{EnvironmentRegionProvider, Region, RegionProvider, StaticRegionProvider},
    retry::RetryConfig,
};

/// Configuration settings for the Lago client
///
/// This struct contains all the configuration options needed to create and customize
/// a Lago client instance, including region settings, credentials, timeouts, and retry policies.
#[derive(Clone)]
pub struct Config {
    pub(crate) region_provider: Arc<dyn RegionProvider>,
    pub(crate) credentials_provider: Arc<dyn CredentialsProvider>,
    pub(crate) timeout: Duration,
    pub(crate) retry_config: RetryConfig,
    pub(crate) user_agent: String,
}

impl Config {
    /// Creates a new configuration builder
    ///
    /// # Returns
    /// A new `ConfigBuilder` instance for constructing a configuration
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    /// Gets the configured region for API requests
    ///
    /// # Returns
    /// A `Result` containing the `Region` or an error if the region cannot be determined
    pub fn region(&self) -> Result<Region, lago_types::error::LagoError> {
        self.region_provider.provider_region()
    }

    /// Gets the configured credentials for API authentication
    ///
    /// # Returns
    /// A `Result` containing the `Credentials` or an error if credentials cannot be loaded
    pub fn credentials(&self) -> Result<Credentials, lago_types::error::LagoError> {
        self.credentials_provider.provider_credentials()
    }

    /// Gets the configured timeout duration for HTTP requests
    ///
    /// # Returns
    /// The timeout duration
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Gets the configured retry settings
    ///
    /// # Returns
    /// A reference to the retry configuration
    pub fn retry_config(&self) -> &RetryConfig {
        &self.retry_config
    }

    /// Gets the configured user agent string
    ///
    /// # Returns
    /// The user agent string used for HTTP requests
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }
}

impl Default for Config {
    /// Creates a default configuration using environment-based providers
    ///
    /// This will attempt to load region and credentials from environment variables,
    /// with sensible defaults for timeout, retry settings, and user agent.
    fn default() -> Self {
        Self {
            region_provider: Arc::new(EnvironmentRegionProvider::new()),
            credentials_provider: Arc::new(EnvironmentCredentialsProvider::new()),
            timeout: Duration::from_secs(30),
            retry_config: RetryConfig::default(),
            user_agent: format!("lago-rust-client/{}", env!("CARGO_PKG_VERSION")),
        }
    }
}

/// Builder for creating customized configuration instances
///
/// This builder allows you to configure various aspects of the Lago client
/// such as region, credentials, timeout, retry behavior, and user agent.
pub struct ConfigBuilder {
    region_provider: Option<Arc<dyn RegionProvider>>,
    credentials_provider: Option<Arc<dyn CredentialsProvider>>,
    timeout: Option<Duration>,
    retry_config: Option<RetryConfig>,
    user_agent: Option<String>,
}

impl ConfigBuilder {
    /// Creates a new configuration builder with default values
    ///
    /// # Returns
    /// A new `ConfigBuilder` instance
    pub fn new() -> Self {
        Self {
            region_provider: None,
            credentials_provider: None,
            timeout: None,
            retry_config: None,
            user_agent: None,
        }
    }

    /// Sets the region for API requests
    ///
    /// # Arguments
    /// * `region` - The region to use for API requests
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn region(mut self, region: Region) -> Self {
        self.region_provider = Some(Arc::new(StaticRegionProvider::new(region)));
        self
    }

    /// Sets a custom region provider
    ///
    /// # Arguments
    /// * `provider` - The region provider to use
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn region_provider(mut self, provider: Arc<dyn RegionProvider>) -> Self {
        self.region_provider = Some(provider);
        self
    }

    /// Sets the credentials for API authentication
    ///
    /// # Arguments
    /// * `credentials` - The credentials to use for authentication
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials_provider = Some(Arc::new(StaticCredentialsProvider::new(credentials)));
        self
    }

    /// Sets a custom credentials provider
    ///
    /// # Arguments
    /// * `provider` - The credentials provider to use
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn credentials_provider(mut self, provider: Arc<dyn CredentialsProvider>) -> Self {
        self.credentials_provider = Some(provider);
        self
    }

    /// Sets the timeout duration for HTTP requests
    ///
    /// # Arguments
    /// * `timeout` - The timeout duration
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Sets the retry configuration
    ///
    /// # Arguments
    /// * `retry_config` - The retry configuration to use
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn retry_config(mut self, retry_config: RetryConfig) -> Self {
        self.retry_config = Some(retry_config);
        self
    }

    /// Sets the user agent string for HTTP requests
    ///
    /// # Arguments
    /// * `user_agent` - The user agent string
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = Some(user_agent);
        self
    }

    /// Builds the final configuration instance
    ///
    /// Any unset values will use the defaults from `Config::default()`.
    ///
    /// # Returns
    /// A new `Config` instance with the specified settings
    pub fn build(self) -> Config {
        let default_config = Config::default();

        Config {
            region_provider: self
                .region_provider
                .unwrap_or(default_config.region_provider),
            credentials_provider: self
                .credentials_provider
                .unwrap_or(default_config.credentials_provider),
            timeout: self.timeout.unwrap_or(default_config.timeout),
            retry_config: self.retry_config.unwrap_or(default_config.retry_config),
            user_agent: self.user_agent.unwrap_or(default_config.user_agent),
        }
    }
}

impl Default for ConfigBuilder {
    /// Creates a default configuration builder
    ///
    /// This is equivalent to calling `ConfigBuilder::new()`.
    fn default() -> Self {
        Self::new()
    }
}
