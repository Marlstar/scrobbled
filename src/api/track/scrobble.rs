use crate::api::Args;
use crate::Scrobble as ActualScrobble;
use crate::session::SessionToken;

pub fn scrobble(scrobble: &ActualScrobble, session: &SessionToken) -> String {
    Args::new()
        .push("method", "track.scrobble")
        .push("track", &scrobble.track)
        .push("artist", &scrobble.artist)
        .push("timestamp", scrobble.timestamp.unwrap_or(chrono::Local::now()).to_utc().timestamp())
        .push_maybe("album", scrobble.album.as_ref())
        .push("sk", session)
        .push_api_key()
        .push_signature()
        .build()
}

/// A collection of `ScrobbleResult`s
#[derive(Debug, serde::Deserialize)]
pub struct Scrobbles {
    #[serde(rename = "@accepted")] pub accepted_count: usize,
    #[serde(rename = "@ignored")] pub ignored_count: usize,
    #[serde(rename = "scrobble")] pub scrobbles: Vec<ScrobbleResult>,
}

/// The API response for a scrobble
#[derive(Debug, serde::Deserialize)]
#[serde(rename = "scrobble")]
pub struct ScrobbleResult {
    pub track: Track,
    pub artist: Artist,
    pub album: Album,
    #[serde(rename = "albumArtist")]
    pub album_artist: AlbumArtist,
    pub timestamp: Timestamp,
    #[serde(rename = "ignoredMessage")]
    pub ignored_message: IgnoredMessage,
}

macro_rules! scrobble_part {
    ($name:ident, $type:ident $(, ($attrname:expr, $attrfield:ident, $attrtype:ident))+) => {
        #[derive(Debug, serde::Deserialize)]
        pub struct $name {
            $(#[serde(rename = $attrname)] pub $attrfield: $attrtype,)+
            #[serde(rename = "#text")] pub inner: $type,
        }
        impl std::ops::Deref for $name {
            type Target = $type;
            fn deref(&self) -> &Self::Target { &self.inner }
        }
    };

    ($name:ident, $type:ident) => {
        #[derive(Debug, serde::Deserialize)]
        pub struct $name(pub $type);
        impl std::ops::Deref for $name {
            type Target = $type;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
    };
}

scrobble_part!(Track, String, ("@corrected", corrected, bool));
scrobble_part!(Artist, String, ("@corrected", corrected, bool));
scrobble_part!(Album, String, ("@corrected", corrected, bool));
scrobble_part!(AlbumArtist, AlbumArtistType, ("@corrected", corrected, bool)); type AlbumArtistType = Option<String>;
scrobble_part!(Timestamp, usize);
scrobble_part!(IgnoredMessage, IgnoredMessageType, ("@code", code, usize)); type IgnoredMessageType = Option<String>;
