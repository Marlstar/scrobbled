use std::sync::LazyLock;
use std::sync::RwLock;

pub(crate) static API_KEY: LazyLock<RwLock<String>> = LazyLock::new(|| RwLock::new(String::from("")));
pub(crate) static API_SECRET: LazyLock<RwLock<String>> = LazyLock::new(|| RwLock::new(String::from("")));

pub(crate) fn get_api_key() -> String { API_KEY.read().unwrap().clone() }
pub(crate) fn get_api_secret() -> String { API_SECRET.read().unwrap().clone() }

/// Set the Last.fm API key to use when making requests
pub fn set_api_key(key: &str) { *API_KEY.write().unwrap() = key.to_string(); }
/// Set the Last.fm API secret to use when making authenticated requests
pub fn set_api_secret(secret: &str) { *API_SECRET.write().unwrap() = secret.to_string(); }
