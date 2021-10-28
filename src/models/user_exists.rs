/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */

/// UserExists : Status object representing if a queried user by username or userId exists or not. This model is primarily used by the `/auth/exists` endpoint, which in turn is used during registration. Please see the documentation on that endpoint for more information on usage.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserExists {
    /// Status if a user exist with that username or userId.
    #[serde(rename = "userExists")]
    pub user_exists: bool,
}

impl UserExists {
    /// Status object representing if a queried user by username or userId exists or not. This model is primarily used by the `/auth/exists` endpoint, which in turn is used during registration. Please see the documentation on that endpoint for more information on usage.
    pub fn new(user_exists: bool) -> UserExists {
        UserExists {
            user_exists,
        }
    }
}


