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
pub struct CreateSubscriptionRequest {
    /// 
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 
    #[serde(rename = "subject")]
    pub subject: serde_json::Value,
    /// 
    #[serde(rename = "notification")]
    pub notification: serde_json::Value,
    /// 
    #[serde(rename = "expires", skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    /// 
    #[serde(rename = "throttling", skip_serializing_if = "Option::is_none")]
    pub throttling: Option<i32>,
}

impl CreateSubscriptionRequest {
    pub fn new(subject: serde_json::Value, notification: serde_json::Value) -> CreateSubscriptionRequest {
        CreateSubscriptionRequest {
            description: None,
            subject,
            notification,
            expires: None,
            throttling: None,
        }
    }
}


