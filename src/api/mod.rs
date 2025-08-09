pub static API_URL: &str = "https://ws.audioscrobbler.com/2.0/";
mod signature; pub use signature::Signature;
mod args; pub use args::Args;

pub mod auth;

pub type APIResult<T> = Result<T, APIError>;

#[derive(Debug)]
pub enum APIError {
    ResponseNotOK(u16),
    Deserialisation(serde_json::Error),
}
impl From<serde_json::Error> for APIError {
    fn from(value: serde_json::Error) -> Self { Self::Deserialisation(value) }
}

#[macro_export]
macro_rules! run {
    ($namespace:ident, $method:ident, $outtype:ident, $reqtype:ident, $ws:expr $(,$arg:expr)*) => { async {
        let req = format!(
            "{}?{}&format=json",
            $crate::api::API_URL,
            $crate::api::$namespace::$method::$method($($arg),+)
        );

        // TODO: don't unwrap here
        let result = $ws.execute($ws.$reqtype(req).build().unwrap()).await.unwrap();
        let status_code = result.status().as_u16();
        let out: $crate::api::APIResult<$crate::api::$namespace::$method::$outtype> = if status_code == 200 {
            Ok(serde_json::from_str(&result.text().await.unwrap())?)
        } else {
            Err($crate::api::APIError::ResponseNotOK(status_code))
        };
        out
    }}
} pub use run;
