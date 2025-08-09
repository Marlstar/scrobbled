#[derive(Debug, from_variants::FromVariants)]
pub enum OAuthError {
    WebserverFailed,
    IO(std::io::Error),
}
