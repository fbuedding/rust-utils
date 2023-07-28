/*
 * FIWARE-NGSI v2 Specification
 *
 * TODO: Add a description
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListEntityTypesResponse {
    /// 
    #[serde(rename = "type")]
    pub r#type: String,
    /// 
    #[serde(rename = "attrs")]
    pub attrs: serde_json::Value,
    /// 
    #[serde(rename = "count")]
    pub count: i32,
}

impl ListEntityTypesResponse {
    pub fn new(r#type: String, attrs: serde_json::Value, count: i32) -> ListEntityTypesResponse {
        ListEntityTypesResponse {
            r#type,
            attrs,
            count,
        }
    }
}


