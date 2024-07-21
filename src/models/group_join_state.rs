/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GroupJoinState {
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "invite")]
    Invite,
    #[serde(rename = "request")]
    Request,
    #[serde(rename = "open")]
    Open,

}

impl std::fmt::Display for GroupJoinState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Closed => write!(f, "closed"),
            Self::Invite => write!(f, "invite"),
            Self::Request => write!(f, "request"),
            Self::Open => write!(f, "open"),
        }
    }
}

impl Default for GroupJoinState {
    fn default() -> GroupJoinState {
        Self::Closed
    }
}

