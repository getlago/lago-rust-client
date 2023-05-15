/*
 * Lago API documentation
 *
 * Lago API allows your application to push customer information and metrics (events) from your application to the billing application.
 *
 * The version of the OpenAPI document: 0.32.0-beta
 * Contact: tech@getlago.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvoiceMetadataObject {
    #[serde(rename = "lago_id", skip_serializing_if = "Option::is_none")]
    pub lago_id: Option<uuid::Uuid>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl InvoiceMetadataObject {
    pub fn new() -> InvoiceMetadataObject {
        InvoiceMetadataObject {
            lago_id: None,
            key: None,
            value: None,
            created_at: None,
        }
    }
}


