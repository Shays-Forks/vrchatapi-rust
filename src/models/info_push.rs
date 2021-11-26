/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InfoPush {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "releaseStatus")]
    pub release_status: crate::models::ReleaseStatus,
    #[serde(rename = "priority")]
    pub priority: i32,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "data")]
    pub data: Box<crate::models::InfoPushData>,
    /// Unknown usage, MD5
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

impl InfoPush {
    pub fn new(id: String, is_enabled: bool, release_status: crate::models::ReleaseStatus, priority: i32, tags: Vec<String>, data: crate::models::InfoPushData, hash: String, created_at: String, updated_at: String) -> InfoPush {
        InfoPush {
            id,
            is_enabled,
            release_status,
            priority,
            tags,
            data: Box::new(data),
            hash,
            created_at,
            updated_at,
            start_date: None,
            end_date: None,
        }
    }
}

