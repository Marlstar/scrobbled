#![allow(clippy::needless_return)]

pub mod auth;

pub(crate) mod secrets;
pub use secrets::{set_api_key, set_api_secret};
