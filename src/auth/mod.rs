pub mod handler;
pub use handler::OAuthHandler;

mod error;
pub use error::OAuthError;

pub async fn get_token() -> Result<String, OAuthError> {
    OAuthHandler::builder().build().await
        .auth().await
}

