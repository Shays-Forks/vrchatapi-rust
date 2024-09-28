/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UserState : * \"online\" User is online in VRChat * \"active\" User is online, but not in VRChat * \"offline\" User is offline  Always offline when returned through `getCurrentUser` (/auth/user).
/// * \"online\" User is online in VRChat * \"active\" User is online, but not in VRChat * \"offline\" User is offline  Always offline when returned through `getCurrentUser` (/auth/user).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserState {
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "online")]
    Online,
}

impl std::fmt::Display for UserState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Offline => write!(f, "offline"),
            Self::Active => write!(f, "active"),
            Self::Online => write!(f, "online"),
        }
    }
}

impl Default for UserState {
    fn default() -> UserState {
        Self::Offline
    }
}
