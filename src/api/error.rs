pub type APIResult<T> = Result<T, APIError>;

#[derive(Debug, from_variants::FromVariants, thiserror::Error)]
pub enum APIError {
    #[error("API request did not return a 200 OK")]
    ResponseNotOK(u16),

    #[error("failed to deserialise API response")]
    Deserialisation(serde_json::Error),

    #[error("failed to communicate with the API")]
    Reqwest(reqwest::Error),
}
