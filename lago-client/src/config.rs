use std::time::Duration;
use std::sync::Arc;

use super::{
  region::Region,
  retry::RetryConfig,
  credentials::{
    CredentialsProvider,
    Credentials,
    EnvironmentCredentialsProvider,
    StaticCredentialsProvider,
  },
};

#[derive(Clone)]
pub struct Config {
  pub(crate) region: Region,
  pub(crate) credentials_provider: Arc<dyn CredentialsProvider>,
  pub(crate) timeout: Duration,
  pub(crate) retry_config: RetryConfig,
  pub(crate) user_agent: String,
}

impl Config {
  pub fn builder() -> ConfigBuilder {
    ConfigBuilder::new()
  }

  pub fn region(&self) -> &Region {
    &self.region
  }

  pub fn credentials(&self) -> Result<Credentials, lago_types::LagoError> {
    self.credentials_provider.provider_credentials()
  }

  pub fn timeout(&self) -> Duration {
    self.timeout
  }

  pub fn retry_config(&self) -> &RetryConfig {
    &self.retry_config
  }

  pub fn user_agent(&self) -> &str {
    &self.user_agent
  }
}

impl Default for Config {
  fn default() -> Self {
    Self {
      region: Region::default(),
      credentials_provider: Arc::new(EnvironmentCredentialsProvider::new()),
      timeout: Duration::from_secs(30),
      retry_config: RetryConfig::default(),
      user_agent: format!("lago-rust-client/{}", env!("CARGO_PKG_VERSION")),
    }
  }
}

pub struct ConfigBuilder {
  region: Option<Region>,
  credentials_provider: Option<Arc<dyn CredentialsProvider>>,
  timeout: Option<Duration>,
  retry_config: Option<RetryConfig>,
  user_agent: Option<String>,
}

impl ConfigBuilder {
  pub fn new() -> Self {
    Self {
      region: None,
      credentials_provider: None,
      timeout: None,
      retry_config: None,
      user_agent: None,
    }
  }

  pub fn region(mut self, region: Region) -> Self {
    self.region = Some(region);
    self
  }

  pub fn credentials(mut self, credentials: Credentials) -> Self {
    self.credentials_provider = Some(Arc::new(StaticCredentialsProvider::new(credentials)));
    self
  }

  pub fn credentials_provider(mut self, provider: Arc<dyn CredentialsProvider>) -> Self {
    self.credentials_provider = Some(provider);
    self
  }

  pub fn timeout(mut self, timeout: Duration) -> Self {
    self.timeout = Some(timeout);
    self
  }

  pub fn retry_config(mut self, retry_config: RetryConfig) -> Self {
    self.retry_config = Some(retry_config);
    self
  }

  pub fn user_agent(mut self, user_agent: String) -> Self {
    self.user_agent = Some(user_agent);
    self
  }

  pub fn build(self) -> Config {
    let default_config = Config::default();

    Config {
      region: self.region.unwrap_or(default_config.region),
      credentials_provider: self.credentials_provider.unwrap_or(default_config.credentials_provider),
      timeout: self.timeout.unwrap_or(default_config.timeout),
      retry_config: self.retry_config.unwrap_or(default_config.retry_config),
      user_agent: self.user_agent.unwrap_or(default_config.user_agent),
    }
  }
}

impl Default for ConfigBuilder {
  fn default() -> Self {
    Self::new()
  }
}