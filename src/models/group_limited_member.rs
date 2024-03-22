/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GroupLimitedMember {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Whether the user is representing the group. This makes the group show up above the name tag in-game.
    #[serde(rename = "isRepresenting", skip_serializing_if = "Option::is_none")]
    pub is_representing: Option<bool>,
    #[serde(rename = "roleIds", skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<Vec<String>>,
    #[serde(rename = "mRoleIds", skip_serializing_if = "Option::is_none")]
    pub m_role_ids: Option<Vec<String>>,
    #[serde(rename = "joinedAt", skip_serializing_if = "Option::is_none")]
    pub joined_at: Option<String>,
    #[serde(rename = "membershipStatus", skip_serializing_if = "Option::is_none")]
    pub membership_status: Option<String>,
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(rename = "isSubscribedToAnnouncements", skip_serializing_if = "Option::is_none")]
    pub is_subscribed_to_announcements: Option<bool>,
    /// Only visible via the /groups/:groupId/members endpoint, **not** when fetching a specific user.
    #[serde(rename = "createdAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Option<String>>,
    /// Only visible via the /groups/:groupId/members endpoint, **not** when fetching a specific user.
    #[serde(rename = "bannedAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub banned_at: Option<Option<String>>,
    /// Only visible via the /groups/:groupId/members endpoint, **not** when fetching a specific user.
    #[serde(rename = "managerNotes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub manager_notes: Option<Option<String>>,
    #[serde(rename = "lastPostReadAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_post_read_at: Option<Option<String>>,
    #[serde(rename = "hasJoinedFromPurchase", skip_serializing_if = "Option::is_none")]
    pub has_joined_from_purchase: Option<bool>,
}

impl GroupLimitedMember {
    pub fn new() -> GroupLimitedMember {
        GroupLimitedMember {
            id: None,
            group_id: None,
            user_id: None,
            is_representing: None,
            role_ids: None,
            m_role_ids: None,
            joined_at: None,
            membership_status: None,
            visibility: None,
            is_subscribed_to_announcements: None,
            created_at: None,
            banned_at: None,
            manager_notes: None,
            last_post_read_at: None,
            has_joined_from_purchase: None,
        }
    }
}


