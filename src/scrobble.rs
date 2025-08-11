use chrono::DateTime;
use chrono::Local;

/// A scrobble to send to Last.fm
/// (sent via a [`Session`])
pub struct Scrobble {
    pub track: String,
    pub artist: String,
    pub album: Option<String>,
    pub timestamp: DateTime<Local>,
}
