/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */

/// DownloadUrlList : Download links for various development assets.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DownloadUrlList {
    /// Download link for legacy SDK2
    #[serde(rename = "sdk2")]
    pub sdk2: String,
    /// Download link for SDK3 for Avatars
    #[serde(rename = "sdk3-avatars")]
    pub sdk3_avatars: String,
    /// Download link for SDK3 for Worlds
    #[serde(rename = "sdk3-worlds")]
    pub sdk3_worlds: String,
}

impl DownloadUrlList {
    /// Download links for various development assets.
    pub fn new(sdk2: String, sdk3_avatars: String, sdk3_worlds: String) -> DownloadUrlList {
        DownloadUrlList {
            sdk2,
            sdk3_avatars,
            sdk3_worlds,
        }
    }
}


