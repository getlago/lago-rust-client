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
pub struct AddOnsPaginated {
    #[serde(rename = "add_ons")]
    pub add_ons: Vec<crate::models::AddOnObject>,
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::PaginationMeta>,
}

impl AddOnsPaginated {
    pub fn new(add_ons: Vec<crate::models::AddOnObject>, meta: crate::models::PaginationMeta) -> AddOnsPaginated {
        AddOnsPaginated {
            add_ons,
            meta: Box::new(meta),
        }
    }
}


