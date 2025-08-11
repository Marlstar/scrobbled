#[derive(Debug, from_variants::FromVariants, thiserror::Error)]
pub enum OAuthError {
    #[error("OAuth token callback webserver stopped prematurely")]
    /// OAuth token callback webserver stopped prematurely
    WebserverFailed,

    #[error("miscellaneous IO error")]
    /// Miscellaneous IO error
    IO(std::io::Error),
}
