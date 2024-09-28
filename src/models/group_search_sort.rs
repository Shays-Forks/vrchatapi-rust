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
pub enum GroupSearchSort {
    #[serde(rename = "joinedAt:asc")]
    Asc,
    #[serde(rename = "joinedAt:desc")]
    Desc,
}

impl std::fmt::Display for GroupSearchSort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Asc => write!(f, "joinedAt:asc"),
            Self::Desc => write!(f, "joinedAt:desc"),
        }
    }
}

impl Default for GroupSearchSort {
    fn default() -> GroupSearchSort {
        Self::Asc
    }
}