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
pub struct GroupObject {
    #[serde(rename = "lago_id")]
    pub lago_id: uuid::Uuid,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
}

impl GroupObject {
    pub fn new(lago_id: uuid::Uuid, key: String, value: String) -> GroupObject {
        GroupObject {
            lago_id,
            key,
            value,
        }
    }
}


