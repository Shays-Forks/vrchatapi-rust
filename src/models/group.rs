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
pub struct Group {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "shortCode", skip_serializing_if = "Option::is_none")]
    pub short_code: Option<String>,
    #[serde(rename = "discriminator", skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "iconUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<Option<String>>,
    #[serde(rename = "bannerUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub banner_url: Option<Option<String>>,
    #[serde(rename = "privacy", skip_serializing_if = "Option::is_none")]
    pub privacy: Option<models::GroupPrivacy>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "ownerId", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "rules", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rules: Option<Option<String>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    #[serde(rename = "languages", skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(rename = "iconId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<Option<String>>,
    #[serde(rename = "bannerId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub banner_id: Option<Option<String>>,
    #[serde(rename = "memberCount", skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    #[serde(rename = "memberCountSyncedAt", skip_serializing_if = "Option::is_none")]
    pub member_count_synced_at: Option<String>,
    #[serde(rename = "isVerified", skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[serde(rename = "joinState", skip_serializing_if = "Option::is_none")]
    pub join_state: Option<models::GroupJoinState>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "transferTargetId", skip_serializing_if = "Option::is_none")]
    pub transfer_target_id: Option<String>,
    #[serde(rename = "galleries", skip_serializing_if = "Option::is_none")]
    pub galleries: Option<Vec<models::GroupGallery>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "lastPostCreatedAt", skip_serializing_if = "Option::is_none")]
    pub last_post_created_at: Option<String>,
    #[serde(rename = "onlineMemberCount", skip_serializing_if = "Option::is_none")]
    pub online_member_count: Option<i32>,
    #[serde(rename = "membershipStatus", skip_serializing_if = "Option::is_none")]
    pub membership_status: Option<models::GroupMemberStatus>,
    #[serde(rename = "myMember", skip_serializing_if = "Option::is_none")]
    pub my_member: Option<Box<models::GroupMyMember>>,
    /// Only returned if ?includeRoles=true is specified.
    #[serde(rename = "roles", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Option<Vec<models::GroupRole>>>,
}

impl Group {
    pub fn new() -> Group {
        Group {
            id: None,
            name: None,
            short_code: None,
            discriminator: None,
            description: None,
            icon_url: None,
            banner_url: None,
            privacy: None,
            owner_id: None,
            rules: None,
            links: None,
            languages: None,
            icon_id: None,
            banner_id: None,
            member_count: None,
            member_count_synced_at: None,
            is_verified: None,
            join_state: None,
            tags: None,
            transfer_target_id: None,
            galleries: None,
            created_at: None,
            updated_at: None,
            last_post_created_at: None,
            online_member_count: None,
            membership_status: None,
            my_member: None,
            roles: None,
        }
    }
}

