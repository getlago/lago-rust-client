use lago_types::error::LagoError;

/// Represents the different regions where the Lago API can be accessed
///
/// This enum defines the available regions for the Lago API, including
/// predefined regions (US, EU) and custom endpoints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Region {
    Us,
    Eu,
    Custom(String),
}

impl Region {
    /// Returns the API endpoint URL for this region
    ///
    /// # Returns
    /// The base URL string for the API endpoint
    pub fn endpoint(&self) -> &str {
        match self {
            Region::Us => "https://api.getlago.com/api/v1",
            Region::Eu => "https://api.eu.getlago.com/api/v1",
            Region::Custom(url) => url,
        }
    }
}

impl Default for Region {
    /// Returns the default region (US)
    ///
    /// # Returns
    /// `Region::Us` as the default region
    fn default() -> Self {
        Region::Us
    }
}

/// Trait for providing region information to the Lago client
///
/// This trait allows for different methods of region determination,
/// such as static configuration, environment variables, or other sources.
pub trait RegionProvider: Send + Sync {
    /// Provides the region for API requests
    ///
    /// # Returns
    /// A `Result` containing the `Region` or an error if the region cannot be determined
    fn provider_region(&self) -> Result<Region, LagoError>;
}

/// A region provider that uses a static, pre-configured region
///
/// This provider holds a region that was provided at creation time
/// and returns it on every request.
#[derive(Clone)]
pub struct StaticRegionProvider {
    region: Region,
}

impl StaticRegionProvider {
    /// Creates a new static region provider with the given region
    ///
    /// # Arguments
    /// * `region` - The region to use for all requests
    ///
    /// # Returns
    /// A new `StaticRegionProvider` instance
    pub fn new(region: Region) -> Self {
        Self { region }
    }
}

impl RegionProvider for StaticRegionProvider {
    /// Returns the static region
    ///
    /// # Returns
    /// A `Result` containing a clone of the stored region
    fn provider_region(&self) -> Result<Region, LagoError> {
        Ok(self.region.clone())
    }
}

/// A region provider that loads region information from environment variables
///
/// This provider attempts to load the region from the `LAGO_REGION` environment variable
/// first, and falls back to `LAGO_API_URL` for custom endpoints. If neither is set,
/// it returns the default region.
pub struct EnvironmentRegionProvider;

impl EnvironmentRegionProvider {
    /// Creates a new environment region provider
    ///
    /// # Returns
    /// A new `EnvironmentRegionProvider` instance
    pub fn new() -> Self {
        Self
    }
}

impl Default for EnvironmentRegionProvider {
    /// Creates a default environment region provider
    ///
    /// This is equivalent to calling `EnvironmentRegionProvider::new()`.
    fn default() -> Self {
        Self::new()
    }
}

impl RegionProvider for EnvironmentRegionProvider {
    /// Loads region information from environment variables
    ///
    /// This method first checks the `LAGO_REGION` environment variable for predefined
    /// regions ("us", "eu") or treats other values as custom endpoints.
    /// If `LAGO_REGION` is not set, it falls back to `LAGO_API_URL`.
    /// If neither is set, it returns the default region.
    ///
    /// # Returns
    /// A `Result` containing the determined `Region`
    fn provider_region(&self) -> Result<Region, LagoError> {
        match std::env::var("LAGO_REGION") {
            Ok(region_str) => match region_str.to_lowercase().as_str() {
                "us" => Ok(Region::Us),
                "eu" => Ok(Region::Eu),
                _ => Ok(Region::Custom(region_str)),
            },
            Err(_) => match std::env::var("LAGO_API_URL") {
                Ok(url) => Ok(Region::Custom(url)),
                Err(_) => Ok(Region::default()),
            },
        }
    }
}
