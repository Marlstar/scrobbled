#![allow(clippy::needless_return)]

pub mod auth;

pub mod session;
pub use session::Session;

pub mod scrobble;
pub use scrobble::Scrobble;

pub(crate) mod api;

pub(crate) mod secrets;
pub use secrets::{set_api_key, set_api_secret};

// Re-exports
pub use chrono;
