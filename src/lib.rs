#![allow(clippy::needless_return)]

pub mod auth;

pub mod session;
pub use session::{Session, SessionError, SessionResult};

pub mod scrobble;
pub use scrobble::Scrobble;

pub(crate) mod api;
pub use api::{APIError, APIResult};

pub(crate) mod secrets;
pub use secrets::{set_api_key, set_api_secret};

// Re-exports
pub use chrono;
