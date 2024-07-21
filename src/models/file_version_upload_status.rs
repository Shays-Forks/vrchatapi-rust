/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// FileVersionUploadStatus : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileVersionUploadStatus {
    #[serde(rename = "uploadId")]
    pub upload_id: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "nextPartNumber")]
    pub next_part_number: f64,
    #[serde(rename = "maxParts")]
    pub max_parts: f64,
    #[serde(rename = "parts")]
    pub parts: Vec<serde_json::Value>,
    /// Unknown
    #[serde(rename = "etags")]
    pub etags: Vec<serde_json::Value>,
}

impl FileVersionUploadStatus {
    pub fn new(upload_id: String, file_name: String, next_part_number: f64, max_parts: f64, parts: Vec<serde_json::Value>, etags: Vec<serde_json::Value>) -> FileVersionUploadStatus {
        FileVersionUploadStatus {
            upload_id,
            file_name,
            next_part_number,
            max_parts,
            parts,
            etags,
        }
    }
}

