use reqwest::Client;
use crate::auth::OAuthToken;
use crate::api::APIError;
use crate::api;

#[derive(Debug)]
pub struct Session {
    token: OAuthToken,
    session: Option<SessionToken>,
    ws: Client,
}
impl Session {
    pub async fn new(token: OAuthToken) -> reqwest::Result<Self>  {
        Ok(Self {
            token,
            session: None,
            ws: Client::builder().build()?,
        })
    }

    pub async fn start(&mut self) -> api::APIResult<()> {
        let session_token = match api::run!(auth, get_session, GetSession, get, &self.ws, &self.token).await {
            Ok(gs) => SessionToken(gs.session.key),
            Err(APIError::ResponseNotOK(code)) => todo!("handle non-ok api results"),
            Err(APIError::Deserialisation(e)) => todo!("handle deserialisation errors"),
        };
        println!("Session token: {session_token}");
        self.session = Some(session_token);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SessionToken(pub String);
impl std::fmt::Display for SessionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
