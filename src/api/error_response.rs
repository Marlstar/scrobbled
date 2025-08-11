use crate::api::error::APIErrorCode;
use serde::Deserialize;

#[derive(Debug, serde::Deserialize)]
#[serde(rename = "error")]
/// API returned an error but the request was successful
pub struct ErrorResponse {
    #[serde(rename = "@code", deserialize_with = "usize_to_error_code")]
    pub code: APIErrorCode,

    #[serde(rename = "#text")]
    pub message: String,
}

fn usize_to_error_code<'de, D>(de: D) -> Result<APIErrorCode, D::Error>
    where D: serde::Deserializer<'de>
{
    let num = usize::deserialize(de)?;
    Ok(APIErrorCode::from(num))
}
