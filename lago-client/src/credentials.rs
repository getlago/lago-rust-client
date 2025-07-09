use lago_types::error::LagoError;

/// API credentials for authenticating with the Lago API
/// 
/// This struct contains the API key required to authenticate requests to the Lago API.
#[derive(Clone)]
pub struct Credentials {
    api_key: String,
}

impl Credentials {
    /// Creates new credentials with the provided API key
    /// 
    /// # Arguments
    /// * `api_key` - The API key for authentication
    /// 
    /// # Returns
    /// A new `Credentials` instance
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
        }
    }
    
    /// Returns the API key
    /// 
    /// # Returns
    /// A reference to the API key string
    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}

/// Trait for providing credentials to the Lago client
/// 
/// This trait allows for different methods of credential provision,
/// such as static credentials, environment variables, or other sources.
pub trait CredentialsProvider: Send + Sync {
    /// Provides credentials for API authentication
    /// 
    /// # Returns
    /// A `Result` containing `Credentials` or an error if credentials cannot be provided
    fn provider_credentials(&self) -> Result<Credentials, LagoError>;
}

/// A credentials provider that uses static, pre-configured credentials
/// 
/// This provider holds credentials that were provided at creation time
/// and returns them on every request.
#[derive(Clone)]
pub struct StaticCredentialsProvider {
    credentials: Credentials,
}

impl StaticCredentialsProvider {
    /// Creates a new static credentials provider with the given credentials
    /// 
    /// # Arguments
    /// * `credentials` - The credentials to use for all requests
    /// 
    /// # Returns
    /// A new `StaticCredentialsProvider` instance
    pub fn new(credentials: Credentials) -> Self {
        Self { credentials }
    }
}

impl CredentialsProvider for StaticCredentialsProvider {
    /// Returns the static credentials
    /// 
    /// # Returns
    /// A `Result` containing a clone of the stored credentials
    fn provider_credentials(&self) -> Result<Credentials, LagoError> {
        Ok(self.credentials.clone())
    }
}

/// A credentials provider that loads credentials from environment variables
/// 
/// This provider attempts to load the API key from the `LAGO_API_KEY` environment variable.
pub struct EnvironmentCredentialsProvider;

impl EnvironmentCredentialsProvider {
    /// Creates a new environment credentials provider
    /// 
    /// # Returns
    /// A new `EnvironmentCredentialsProvider` instance
    pub fn new() -> Self {
        Self
    }
}

impl Default for EnvironmentCredentialsProvider {
    /// Creates a default environment credentials provider
    /// 
    /// This is equivalent to calling `EnvironmentCredentialsProvider::new()`.
    fn default() -> Self {
        Self::new()
    }
}

impl CredentialsProvider for EnvironmentCredentialsProvider {
    /// Loads credentials from the `LAGO_API_KEY` environment variable
    /// 
    /// # Returns
    /// A `Result` containing `Credentials` loaded from the environment,
    /// or an error if the environment variable is not set
    fn provider_credentials(&self) -> Result<Credentials, LagoError> {
        std::env::var("LAGO_API_KEY")
        .map(Credentials::new)
        .map_err(|_| LagoError::Configuration(
            "LAGO_API_KEY environment variable not found".to_string()
        ))
    }
}