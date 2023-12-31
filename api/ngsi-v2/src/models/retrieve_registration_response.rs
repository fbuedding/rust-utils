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
pub struct RetrieveRegistrationResponse {
    /// 
    #[serde(rename = "id")]
    pub id: String,
    /// 
    #[serde(rename = "description")]
    pub description: String,
    /// 
    #[serde(rename = "dataProvided")]
    pub data_provided: serde_json::Value,
    /// 
    #[serde(rename = "provider")]
    pub provider: serde_json::Value,
    /// 
    #[serde(rename = "expires")]
    pub expires: String,
    /// 
    #[serde(rename = "status")]
    pub status: String,
    /// 
    #[serde(rename = "forwardingInformation")]
    pub forwarding_information: serde_json::Value,
}

impl RetrieveRegistrationResponse {
    pub fn new(id: String, description: String, data_provided: serde_json::Value, provider: serde_json::Value, expires: String, status: String, forwarding_information: serde_json::Value) -> RetrieveRegistrationResponse {
        RetrieveRegistrationResponse {
            id,
            description,
            data_provided,
            provider,
            expires,
            status,
            forwarding_information,
        }
    }
}


