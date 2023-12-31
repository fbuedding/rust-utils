/*
 * FIWARE-NGSI v2 Specification
 *
 * TODO: Add a description
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Options {
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "keyValues")]
    KeyValues,
    #[serde(rename = "values")]
    Values,
    #[serde(rename = "unique")]
    Unique,

}

impl ToString for Options {
    fn to_string(&self) -> String {
        match self {
            Self::Count => String::from("count"),
            Self::KeyValues => String::from("keyValues"),
            Self::Values => String::from("values"),
            Self::Unique => String::from("unique"),
        }
    }
}

impl Default for Options {
    fn default() -> Options {
        Self::Count
    }
}




