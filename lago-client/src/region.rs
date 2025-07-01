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