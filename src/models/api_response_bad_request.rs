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
pub struct ApiResponseBadRequest {
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "error")]
    pub error: String,
}

impl ApiResponseBadRequest {
    pub fn new(status: i32, error: String) -> ApiResponseBadRequest {
        ApiResponseBadRequest {
            status,
            error,
        }
    }
}


