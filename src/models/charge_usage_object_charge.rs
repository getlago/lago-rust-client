/*
 * Lago API documentation
 *
 * Lago API allows your application to push customer information and metrics (events) from your application to the billing application.
 *
 * The version of the OpenAPI document: 0.21.0-beta
 * Contact: tech@getlago.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChargeUsageObjectCharge {
    #[serde(rename = "lago_id", skip_serializing_if = "Option::is_none")]
    pub lago_id: Option<String>,
    /// Charge model type
    #[serde(rename = "charge_model", skip_serializing_if = "Option::is_none")]
    pub charge_model: Option<ChargeModel>,
}

impl ChargeUsageObjectCharge {
    pub fn new() -> ChargeUsageObjectCharge {
        ChargeUsageObjectCharge {
            lago_id: None,
            charge_model: None,
        }
    }
}

/// Charge model type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChargeModel {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "graduated")]
    Graduated,
    #[serde(rename = "package")]
    Package,
    #[serde(rename = "percentage")]
    Percentage,
    #[serde(rename = "volume")]
    Volume,
}

impl Default for ChargeModel {
    fn default() -> ChargeModel {
        Self::Standard
    }
}

