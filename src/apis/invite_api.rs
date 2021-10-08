/*
 * VRChat API Documentation
 *
 *
 * The version of the OpenAPI document: 1.4.1
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `get_invite_message`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInviteMessageError {
    Status400(crate::models::InlineResponse400),
    Status401(crate::models::InlineResponse400),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_invite_messages`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInviteMessagesError {
    Status400(crate::models::InlineResponse400),
    Status401(crate::models::InlineResponse400),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `invite_user`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InviteUserError {
    Status403(crate::models::InlineResponse400),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `request_invite`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestInviteError {
    Status403(crate::models::InlineResponse400),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `reset_invite_message`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResetInviteMessageError {
    Status400(crate::models::InlineResponse400),
    Status401(crate::models::InlineResponse400),
    Status429(crate::models::InlineResponse400),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `respond_invite`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RespondInviteError {
    Status400(crate::models::InlineResponse400),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_invite_message`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateInviteMessageError {
    Status400(crate::models::InlineResponse400),
    Status401(crate::models::InlineResponse400),
    Status429(crate::models::InlineResponse400),
    UnknownValue(serde_json::Value),
}


/// Returns a single Invite Message. This returns the exact same information but less than `getInviteMessages`. Admin Credentials are required to view messages of other users!  Message type refers to a different collection of messages, used during different types of responses.  * `message` = Message during a normal invite * `response` = Message when replying to a message * `request` = Message when requesting an invite * `requestResponse` = Message when replying to a request for invite
pub fn get_invite_message(configuration: &configuration::Configuration, user_id: &str, message_type: &str, message_id: i32) -> Result<crate::models::InviteMessage, Error<GetInviteMessageError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/message/{userId}/{messageType}/{messageId}", configuration.base_path, userId=crate::apis::urlencode(user_id), messageType=crate::apis::urlencode(message_type), messageId=message_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetInviteMessageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of all the users Invite Messages. Admin Credentials are required to view messages of other users!  Message type refers to a different collection of messages, used during different types of responses.  * `message` = Message during a normal invite * `response` = Message when replying to a message * `request` = Message when requesting an invite * `requestResponse` = Message when replying to a request for invite
pub fn get_invite_messages(configuration: &configuration::Configuration, user_id: &str, message_type: &str) -> Result<Vec<crate::models::InviteMessage>, Error<GetInviteMessagesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/message/{userId}/{messageType}", configuration.base_path, userId=crate::apis::urlencode(user_id), messageType=crate::apis::urlencode(message_type));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetInviteMessagesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sends an invite to a user. Returns the Notification of type `invite` that was sent.
pub fn invite_user(configuration: &configuration::Configuration, user_id: &str, invite_request: Option<crate::models::InviteRequest>) -> Result<crate::models::Notification, Error<InviteUserError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/invite/{userId}", configuration.base_path, userId=crate::apis::urlencode(user_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&invite_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<InviteUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Requests an invite from a user. Returns the Notification of type `requestInvite` that was sent.
pub fn request_invite(configuration: &configuration::Configuration, user_id: &str) -> Result<crate::models::Notification, Error<RequestInviteError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/requestInvite/{userId}", configuration.base_path, userId=crate::apis::urlencode(user_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RequestInviteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Resets a single Invite Message back to it's original message, and then returns a list of all of them. Admin Credentials are required to update messages of other users!  Resetting a message respects the rate-limit, but resetting it does not set the rate-limit to 60 like when editing it. It is possible to edit it right after resetting it. Trying to edit a message before the cooldown timer expires results in a 429 Too Fast Error.  Message type refers to a different collection of messages, used during different types of responses.  * `message` = Message during a normal invite * `response` = Message when replying to a message * `request` = Message when requesting an invite * `requestResponse` = Message when replying to a request for invite
pub fn reset_invite_message(configuration: &configuration::Configuration, user_id: &str, message_type: &str, message_id: i32) -> Result<Vec<crate::models::InviteMessage>, Error<ResetInviteMessageError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/message/{userId}/{messageType}/{messageId}", configuration.base_path, userId=crate::apis::urlencode(user_id), messageType=crate::apis::urlencode(message_type), messageId=message_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ResetInviteMessageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sends a world invite to a user.
pub fn respond_invite(configuration: &configuration::Configuration, notification_id: &str, invite_response: Option<crate::models::InviteResponse>) -> Result<crate::models::Notification, Error<RespondInviteError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/invite/{notificationId}/response", configuration.base_path, notificationId=crate::apis::urlencode(notification_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&invite_response);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RespondInviteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a single Invite Message and then returns a list of all of them. Admin Credentials are required to update messages of other users!  Updating a message automatically sets the cooldown timer to 60 minutes. Trying to edit a message before the cooldown timer expires results in a 429 Too Fast Error.  Message type refers to a different collection of messages, used during different types of responses.  * `message` = Message during a normal invite * `response` = Message when replying to a message * `request` = Message when requesting an invite * `requestResponse` = Message when replying to a request for invite
pub fn update_invite_message(configuration: &configuration::Configuration, user_id: &str, message_type: &str, message_id: i32) -> Result<Vec<crate::models::InviteMessage>, Error<UpdateInviteMessageError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/message/{userId}/{messageType}/{messageId}", configuration.base_path, userId=crate::apis::urlencode(user_id), messageType=crate::apis::urlencode(message_type), messageId=message_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateInviteMessageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
