use lago_types::error::LagoError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Region {
    Us,
    Eu,
    Custom(String),
}

impl Region {
    pub fn endpoint(&self) -> &str {
        match self {
            Region::Us => "https://api.getlago.com/api/v1",
            Region::Eu => "https://api.eu.getlago.com/api/v1",
            Region::Custom(url) => url,
        }
    }
}

impl Default for Region {
    fn default() -> Self {
        Region::Us
    }
}

pub trait RegionProvider: Send + Sync {
    fn provider_region(&self) -> Result<Region, LagoError>;
}

#[derive(Clone)]
pub struct StaticRegionProvider {
    region: Region,
}

impl StaticRegionProvider {
    pub fn new(region: Region) -> Self {
        Self { region }
    }
}

impl RegionProvider for StaticRegionProvider {
    fn provider_region(&self) -> Result<Region, LagoError> {
        Ok(self.region.clone())
    }
}

pub struct EnvironmentRegionProvider;

impl EnvironmentRegionProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for EnvironmentRegionProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl RegionProvider for EnvironmentRegionProvider {
    fn provider_region(&self) -> Result<Region, LagoError> {
        match std::env::var("LAGO_REGION") {
            Ok(region_str) => {
                match region_str.to_lowercase().as_str() {
                    "us" => Ok(Region::Us),
                    "eu" => Ok(Region::Eu),
                    _ => Ok(Region::Custom(region_str)),
                }
            }
            Err(_) => {
                match std::env::var("LAGO_API_URL") {
                    Ok(url) => Ok(Region::Custom(url)),
                    Err(_) => Ok(Region::default()),
                }
            }
        }
    }
}