pub static API_URL: &str = "https://ws.audioscrobbler.com/2.0/";
mod signature; pub use signature::Signature;
mod args; pub use args::Args;

pub mod auth;

mod error;
pub use error::{APIError, APIResult};

mod error_response;
pub use error_response::ErrorResponse;

mod lfm;
pub use lfm::{Lfm, Status as LfmStatus};

pub type Response<T> = Result<T, ErrorResponse>;

#[macro_export]
macro_rules! run {
    ($namespace:ident, $method:ident, $outtype:ident, $reqtype:ident, $ws:expr $(,$arg:expr)*) => {{
        let __inner__ = async || -> $crate::api::APIResult<$crate::api::Response<$crate::api::$namespace::$method::$outtype>> {
            let req = format!(
                // raw=true removes the wrapper xml element showing the status
                "{}?{}",
                $crate::api::API_URL,
                $crate::api::$namespace::$method::$method($($arg),+)
            );

            let result = $ws.execute($ws.$reqtype(req).build()?).await?;
            let status_code = result.status().as_u16();

            let out: $crate::api::APIResult<$crate::api::Response<$crate::api::$namespace::$method::$outtype>> = if status_code == 200 {
                let text = result.text().await?;
                let (wrapper, content) = $crate::api::remove_wrapper(&text);

                let output: $crate::api::Response<$crate::api::$namespace::$method::$outtype> = match serde_xml_rs::from_str::<$crate::api::Lfm>(&wrapper)?.status {
                    $crate::api::LfmStatus::Ok => Ok(serde_xml_rs::from_str::<$crate::api::$namespace::$method::$outtype>(&content)?),
                    $crate::api::LfmStatus::Failed => Err(serde_xml_rs::from_str::<$crate::api::ErrorResponse>(&content)?)
                };
                return Ok(output);
            } else {
                // TODO: map these codes into an enum
                Err($crate::api::APIError::ResponseNotOK(status_code))
            };
            out
        };
        __inner__()
    }
}} pub use run;

pub fn remove_wrapper(text: &str) -> (String, String) {
    let mut lines = text.lines().skip(1);
    let first_line = lines.next().unwrap();
    let mut lines: Vec<&str> = lines.collect();
    let last_line = lines.pop().unwrap();
    
    let wrapper = format!("{first_line}\n{last_line}");
    let inner = lines.join("\n");

    return (wrapper, inner);
}

#[macro_export]
macro_rules! failable {
    ($name:ident) => {
        pub enum $name {}
    }
}
