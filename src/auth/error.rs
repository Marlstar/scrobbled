#[derive(Debug)]
pub enum OAuthError {
    WebserverFailed,
    IO(std::io::Error),
}

macro_rules! e {
    ($from:path, $to:ident) => {
        impl From<$from> for OAuthError { fn from(value: $from) -> Self { Self::$to(value) } }
    }
}

e!(std::io::Error, IO);
