pub type APIResult<T> = Result<T, APIError>;

#[derive(Debug, from_variants::FromVariants, thiserror::Error)]
pub enum APIError {
    #[error("API request did not return a 200 OK")]
    /// API request did not return a 200 OK
    ResponseNotOK(u16),

    #[error("failed to deserialise API response")]
    /// Failed to deserialise API response
    Deserialisation(serde_xml_rs::Error),

    #[error("failed to communicate with the API")]
    /// Failed to communicate with the API
    Reqwest(reqwest::Error),

    #[error("API returned an error")]
    /// API returned an error
    ErrorResponse(crate::api::ErrorResponse),
}

macro_rules! generate_api_errorcode_enum {
    ($($num:expr, $name:ident, $doc:expr,)+) => {
        #[derive(Debug, Clone, Copy, Hash)]
        pub enum APIErrorCode {
            $(
                #[doc = $doc]
                $name,
            )*
            Unknown(usize),
        }
        impl From<usize> for APIErrorCode {
            fn from(value: usize) -> Self {
                match value {
                    $($num => Self::$name,)+
                    u => Self::Unknown(u)
                }
            }
        }
        impl std::ops::Deref for APIErrorCode {
            type Target = usize;
            fn deref(&self) -> &Self::Target {
                match self {
                    $(Self::$name => &$num,)+
                    Self::Unknown(u) => &u,
                }
            }
        }
    }
}

generate_api_errorcode_enum!(
    2, InvalidService, "The requested service does not exist",
    3, InvalidMethod, "No method with that name in this package",
    4, AuthenticationFailed, "You do not have permissions to access the requested service",
    5, InvalidFormat, "The requested service doesn't exist in that format",
    6, InvalidParameters, "Your request is missing a required parameter",
    7, InvalidResource, "Invalid resource specified",
    8, OperationFailed, "Something else went wrong",
    9, InvalidSessionKey, "Invalid session key, please reauthenticate",
    10, InvalidAPIKey, "You must be granted a valid key by last.fm",
    11, ServiceOffline, "This service is temporarily offline. Try again later.",
    13, InvalidMethodSignature, "Invalid method signature supplied",
    16, Temporary, "There was a temporary error processing your request. Please try again",
    26, SuspendedAPIKey, "Access for your account has been suspended, please contact Last.fm",
    29, RateLimitExceeded, "Rate limit exceeded - your IP has made too many requests in a short period",
);
