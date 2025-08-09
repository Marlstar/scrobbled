#[derive(Debug, from_variants::FromVariants, thiserror::Error)]
pub enum OAuthError {
    #[error("oauth token callback webserver stopped prematurely")]
    WebserverFailed,

    #[error("miscellaneous IO error")]
    IO(std::io::Error),
}
