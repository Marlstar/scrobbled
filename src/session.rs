use reqwest::Client;
use crate::auth::OAuthToken;
use crate::api::{self, APIError, ErrorResponse};
use crate::Scrobble;
use crate::api::track::scrobble::Scrobbles;

#[derive(Debug)]
/// The main handler for Last.fm API interaction.
/// This is where almost all external crate interaction happens.
pub struct Session {
    token: SessionToken,
    ws: Client,
}
impl Session {
    pub async fn new(token: OAuthToken) -> SessionResult<Self> {
        let ws = Client::builder().build()?;
        let token = SessionToken(api::run!(auth, get_session, api::auth::get_session::Session, get, &ws, &token).await?.key);
        Ok(Self {
            token,
            ws,
        })
    }
    
    /// Scrobble a song to the authenticated user's Last.fm account
    pub async fn scrobble(&self, scrobble: &Scrobble) -> SessionResult<Scrobbles> {
        Ok(api::run!(track, scrobble, Scrobbles, post, &self.ws, scrobble, &self.token).await?)
    }
}

#[derive(Debug, Clone)]
/// A session token for a user
pub struct SessionToken(pub String);
impl std::fmt::Display for SessionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, from_variants::FromVariants)]
pub enum SessionError {
    /// Error communicating with the API endpoint
    Reqwest(reqwest::Error),
    /// Error with the API itself
    API(APIError),
}

pub type SessionResult<T> = Result<T, SessionError>;
