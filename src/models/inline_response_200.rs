/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InlineResponse200 {
    #[serde(rename = "ok")]
    pub ok: bool,
    #[serde(rename = "token")]
    pub token: String,
}

impl InlineResponse200 {
    pub fn new(ok: bool, token: String) -> InlineResponse200 {
        InlineResponse200 {
            ok,
            token,
        }
    }
}


