/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// GroupAccessType : Group access type when the instance type is \"group\"
/// Group access type when the instance type is \"group\"
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GroupAccessType {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "plus")]
    Plus,
    #[serde(rename = "members")]
    Members,
}

impl std::fmt::Display for GroupAccessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Public => write!(f, "public"),
            Self::Plus => write!(f, "plus"),
            Self::Members => write!(f, "members"),
        }
    }
}

impl Default for GroupAccessType {
    fn default() -> GroupAccessType {
        Self::Public
    }
}