#[derive(Debug, serde::Deserialize)]
#[serde(rename = "error")]
pub struct ErrorResponse {
    #[serde(rename = "@code")]
    code: usize,

    #[serde(rename = "#text")]
    message: String,
}
