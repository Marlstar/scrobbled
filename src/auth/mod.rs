pub mod handler;
pub use handler::OAuthHandler;

mod error;
pub use error::OAuthError;

pub async fn get_token() -> Result<OAuthToken, OAuthError> {
    OAuthHandler::builder().build().await
        .auth().await
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OAuthToken(pub String);
