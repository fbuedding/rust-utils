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
pub struct UpdateOrAppendEntityAttributesRequest {
    /// 
    #[serde(rename = "ambientNoise")]
    pub ambient_noise: serde_json::Value,
}

impl UpdateOrAppendEntityAttributesRequest {
    pub fn new(ambient_noise: serde_json::Value) -> UpdateOrAppendEntityAttributesRequest {
        UpdateOrAppendEntityAttributesRequest {
            ambient_noise,
        }
    }
}


