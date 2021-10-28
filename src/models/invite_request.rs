/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InviteRequest {
    #[serde(rename = "instanceId")]
    pub instance_id: String,
}

impl InviteRequest {
    pub fn new(instance_id: String) -> InviteRequest {
        InviteRequest {
            instance_id,
        }
    }
}


