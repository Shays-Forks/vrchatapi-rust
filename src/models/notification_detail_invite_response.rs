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
pub struct NotificationDetailInviteResponse {
    #[serde(rename = "inResponseTo")]
    pub in_response_to: String,
    #[serde(rename = "responseMessage")]
    pub response_message: String,
}

impl NotificationDetailInviteResponse {
    pub fn new(
        in_response_to: String,
        response_message: String,
    ) -> NotificationDetailInviteResponse {
        NotificationDetailInviteResponse {
            in_response_to,
            response_message,
        }
    }
}
