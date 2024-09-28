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
pub struct CreateGroupGalleryRequest {
    /// Name of the gallery.
    #[serde(rename = "name")]
    pub name: String,
    /// Description of the gallery.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the gallery is members only.
    #[serde(rename = "membersOnly", skip_serializing_if = "Option::is_none")]
    pub members_only: Option<bool>,
    #[serde(
        rename = "roleIdsToView",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub role_ids_to_view: Option<Option<Vec<String>>>,
    #[serde(
        rename = "roleIdsToSubmit",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub role_ids_to_submit: Option<Option<Vec<String>>>,
    #[serde(
        rename = "roleIdsToAutoApprove",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub role_ids_to_auto_approve: Option<Option<Vec<String>>>,
    #[serde(
        rename = "roleIdsToManage",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub role_ids_to_manage: Option<Option<Vec<String>>>,
}

impl CreateGroupGalleryRequest {
    pub fn new(name: String) -> CreateGroupGalleryRequest {
        CreateGroupGalleryRequest {
            name,
            description: None,
            members_only: None,
            role_ids_to_view: None,
            role_ids_to_submit: None,
            role_ids_to_auto_approve: None,
            role_ids_to_manage: None,
        }
    }
}
