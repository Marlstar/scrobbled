#[derive(Debug, serde::Deserialize)]
pub struct Lfm {
    #[serde(rename = "@status")]
    pub status: Status
}

#[derive(Debug, serde::Deserialize)]
pub enum Status {
    #[serde(rename = "ok")] Ok,
    #[serde(rename = "failed")] Failed,
}
