pub mod handler;
pub use handler::OAuthHandler;
#[cfg(feature = "custom-callback")]
pub use handler::callback_router;

mod error;
pub use error::OAuthError;

pub async fn get_token() -> Result<OAuthToken, OAuthError> {
    OAuthHandler::builder().build().await
        .auth().await
}

#[cfg(feature = "custom-callback")]
pub async fn get_token_with_callback(cb: &str) -> Result<OAuthToken, OAuthError> {
    OAuthHandler::builder().build().await
        .auth_custom(cb).await
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// Single-use authentication token.
/// Used to obtain a session.
pub struct OAuthToken(pub String);
impl std::fmt::Display for OAuthToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
