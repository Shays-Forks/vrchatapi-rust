/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// File : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "extension")]
    pub extension: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "mimeType")]
    pub mime_type: models::MimeType,
    #[serde(rename = "name")]
    pub name: String,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "versions")]
    pub versions: Vec<models::FileVersion>,
}

impl File {
    pub fn new(extension: String, id: String, mime_type: models::MimeType, name: String, owner_id: String, tags: Vec<String>, versions: Vec<models::FileVersion>) -> File {
        File {
            extension,
            id,
            mime_type,
            name,
            owner_id,
            tags,
            versions,
        }
    }
}

