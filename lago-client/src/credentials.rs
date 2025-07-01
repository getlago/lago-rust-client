#[derive(Clone)]
pub struct Credentials {
  api_key: String,
}

impl Credentials {
  pub fn new(api_key: impl Into<String>) -> Self {
    Self {
      api_key: api_key.into(),
    }
  }

  pub fn api_key(&self) -> &str {
    &self.api_key
  }
}

pub trait CredentialsProvider: Send + Sync {
  fn provider_credentials(&self) -> Result<Credentials, lago_types::LagoError>;
}

#[derive(Clone)]
pub struct StaticCredentialsProvider {
  credentials: Credentials,
}

impl StaticCredentialsProvider {
  pub fn new(credentials: Credentials) -> Self {
    Self { credentials }
  }
}

impl CredentialsProvider for StaticCredentialsProvider {
  fn provider_credentials(&self) -> Result<Credentials, lago_types::LagoError> {
    Ok(self.credentials.clone())
  }
}

pub struct EnvironmentCredentialsProvider;

impl EnvironmentCredentialsProvider {
  pub fn new() -> Self {
    Self
  }
}

impl Default for EnvironmentCredentialsProvider {
  fn default() -> Self {
    Self::new()
  }
}

impl CredentialsProvider for EnvironmentCredentialsProvider {
  fn provider_credentials(&self) -> Result<Credentials, lago_types::LagoError> {
    std::env::var("LAGO_API_KEY")
      .map(Credentials::new)
      .map_err(|_| lago_types::LagoError::Configuration(
        "LAGO_API_KEY environment variable not found".to_string()
      ))
  }
}