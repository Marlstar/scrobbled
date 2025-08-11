use crate::api::Args;
use crate::auth::OAuthToken;

pub fn get_session(token: &OAuthToken) -> String {
    Args::new()
        .push("method", "auth.getSession")
        .push_token(token)
        .push_api_key()
        .push_signature()
        .build()
}

/// The API response for `auth.getsession`
#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct Session {
    #[serde(rename = "name")]
    /// Last.fm username
    pub username: String,
    /// Session key
    pub key: String,
    pub subscriber: usize,
}
