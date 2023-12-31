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
pub struct ReplaceAllEntityAttributesRequest {
    /// 
    #[serde(rename = "temperature")]
    pub temperature: serde_json::Value,
    /// 
    #[serde(rename = "seatNumber")]
    pub seat_number: serde_json::Value,
}

impl ReplaceAllEntityAttributesRequest {
    pub fn new(temperature: serde_json::Value, seat_number: serde_json::Value) -> ReplaceAllEntityAttributesRequest {
        ReplaceAllEntityAttributesRequest {
            temperature,
            seat_number,
        }
    }
}


