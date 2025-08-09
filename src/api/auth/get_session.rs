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

pub use deserialise::GetSession;
mod deserialise {
    #[derive(serde::Deserialize, Debug)]
    pub struct GetSession {
        pub session: Session,
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct Session {
        pub name: String,
        pub key: String,
        pub subscriber: usize,
    }
}
