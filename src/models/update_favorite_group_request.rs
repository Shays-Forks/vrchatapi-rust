/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateFavoriteGroupRequest {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<models::FavoriteGroupVisibility>,
    /// Tags on FavoriteGroups are believed to do nothing.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl UpdateFavoriteGroupRequest {
    pub fn new() -> UpdateFavoriteGroupRequest {
        UpdateFavoriteGroupRequest {
            display_name: None,
            visibility: None,
            tags: None,
        }
    }
}
