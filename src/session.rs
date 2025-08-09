use reqwest::Client;
use crate::auth::OAuthToken;
use crate::api::APIError;
use crate::api;

#[derive(Debug)]
pub struct Session {
    token: SessionToken,
    ws: Client,
}
impl Session {
    pub async fn new(token: OAuthToken) -> SessionResult<Self> {
        let ws = Client::builder().build()?;
        let token = SessionToken(api::run!(auth, get_session, GetSession, get, &ws, &token).await?.session.key);
        Ok(Self {
            token,
            ws,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SessionToken(pub String);
impl std::fmt::Display for SessionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub enum SessionError {
    Reqwest(reqwest::Error),
    API(APIError),
}
impl From<reqwest::Error> for SessionError {
    fn from(value: reqwest::Error) -> Self { Self::Reqwest(value) }
}
impl From<APIError> for SessionError {
    fn from(value: APIError) -> Self { Self::API(value) }
}

pub type SessionResult<T> = Result<T, SessionError>;
