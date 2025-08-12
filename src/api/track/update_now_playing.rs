use crate::api::Args;
use crate::Scrobble as ActualScrobble;
use crate::session::SessionToken;

pub fn update_now_playing(nowplaying: &ActualScrobble, session: &SessionToken) -> String {
    Args::new()
        .push("method", "track.updatenowplaying")
        .push("track", &nowplaying.track)
        .push("artist", &nowplaying.artist)
        .push_maybe("album", nowplaying.album.as_ref())
        .push("sk", session)
        .push_api_key()
        .push_signature()
        .build()
}

/// The API response for `track.updatenowplaying`
#[derive(Debug, serde::Deserialize)]
#[serde(rename = "nowplaying")]
pub struct NowPlaying {
    pub track: Track,
    pub artist: Artist,
    pub album: Album,
    #[serde(rename = "albumArtist")]
    pub album_artist: AlbumArtist,
    #[serde(rename = "ignoredMessage")]
    pub ignored_message: IgnoredMessage,
}

macro_rules! nowplaying_part {
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

nowplaying_part!(Track, String, ("@corrected", corrected, bool));
nowplaying_part!(Artist, String, ("@corrected", corrected, bool));
nowplaying_part!(Album, String, ("@corrected", corrected, bool));
nowplaying_part!(AlbumArtist, AlbumArtistType, ("@corrected", corrected, bool)); type AlbumArtistType = Option<String>;
nowplaying_part!(IgnoredMessage, IgnoredMessageType, ("@code", code, usize)); type IgnoredMessageType = Option<String>;
