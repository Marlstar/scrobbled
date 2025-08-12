use chrono::DateTime;
use chrono::Local;

/// A scrobble to send to Last.fm
/// (sent via a [`Session`])
pub struct Scrobble {
    pub track: String,
    pub artist: String,
    pub album: Option<String>,
    pub timestamp: Option<DateTime<Local>>,
}
impl Scrobble {
    pub fn new<S>(track: S, artist: S, album: Option<S>, timestamp: Option<DateTime<Local>>) -> Self where S: ToString {
        Self {
            track: track.to_string(),
            artist: artist.to_string(),
            album: album.as_ref().map(S::to_string),
            timestamp,
        }
    }
}
