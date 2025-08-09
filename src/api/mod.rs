pub static API_URL: &str = "https://ws.audioscrobbler.com/2.0/";
mod signature; pub use signature::Signature;
mod args; pub use args::Args;

pub mod auth;

pub type APIResult<T> = Result<T, APIError>;

#[derive(Debug, from_variants::FromVariants)]
pub enum APIError {
    ResponseNotOK(u16),
    Deserialisation(serde_json::Error),
    Reqwest(reqwest::Error),
}

#[macro_export]
macro_rules! run {
    ($namespace:ident, $method:ident, $outtype:ident, $reqtype:ident, $ws:expr $(,$arg:expr)*) => {{
        let __inner__ = async || -> $crate::api::APIResult<$crate::api::$namespace::$method::$outtype> {
            let req = format!(
                "{}?{}&format=json",
                $crate::api::API_URL,
                $crate::api::$namespace::$method::$method($($arg),+)
            );

            let result = $ws.execute($ws.$reqtype(req).build()?).await?;
            let status_code = result.status().as_u16();
            let out: $crate::api::APIResult<$crate::api::$namespace::$method::$outtype> = if status_code == 200 {
                serde_json::from_str(&result.text().await?)
                    .map_err($crate::api::APIError::Deserialisation)
            } else {
                Err($crate::api::APIError::ResponseNotOK(status_code))
            };
            out
        };
        __inner__()
    }
}} pub use run;
