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

#[derive(serde::Deserialize, Debug)]
pub struct Session {
    #[serde(rename = "name")]
    pub username: String,
    pub key: String,
    pub subscriber: usize,
}
