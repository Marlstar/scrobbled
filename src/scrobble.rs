use chrono::DateTime;
use chrono::Local;

pub struct Scrobble {
    pub track: String,
    pub artist: String,
    pub album: Option<String>,
    pub timestamp: DateTime<Local>,
}
