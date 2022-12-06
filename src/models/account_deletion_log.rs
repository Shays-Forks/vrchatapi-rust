/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountDeletionLog {
    /// Typically \"Deletion requested\" or \"Deletion canceled\". Other messages like \"Deletion completed\" may exist, but are these are not possible to see as a regular user.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// When the deletion is scheduled to happen, standard is 14 days after the request.
    #[serde(rename = "deletionScheduled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub deletion_scheduled: Option<Option<String>>,
    /// Date and time of the deletion request.
    #[serde(rename = "dateTime", skip_serializing_if = "Option::is_none")]
    pub date_time: Option<String>,
}

impl AccountDeletionLog {
    pub fn new() -> AccountDeletionLog {
        AccountDeletionLog {
            message: None,
            deletion_scheduled: None,
            date_time: None,
        }
    }
}


