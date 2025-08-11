use std::collections::HashMap;
use crate::secrets::{get_api_key, get_api_secret};
use crate::auth::OAuthToken;

use super::Signature;

pub type Arg = (Key, Value);
pub type Key = &'static str;
pub type Value = String;

pub struct Args(HashMap<&'static str, String>);
impl From<Vec<Arg>> for Args {
    fn from(value: Vec<Arg>) -> Self {
        Self(HashMap::from_iter(value))
    }
}
impl FromIterator<Arg> for Args {
    fn from_iter<T: IntoIterator<Item = Arg>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter))
    }
}
impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}
impl Args {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn push(&mut self, k: Key, v: impl ToString) -> &mut Self {
        self.0.insert(k, v.to_string());
        self
    }

    /// Push a value, if `Some`
    pub fn push_maybe(&mut self, k: Key, v: Option<impl ToString>) -> &mut Self {
        if let Some(val) = v {
            self.push(k, val)
        } else { self }
    }

    pub fn build(&self) -> String {
        return self.0.iter()
            .map(|(k,v)| format!("{k}={v}"))
            .collect::<Vec<String>>()
            .join("&");
    }

    pub fn signature(&self) -> String {
        Signature::from_iter(self.0.iter()
            .map(|(k,v)| (*k, v.clone()))
            .filter(|(k,_)| *k != "api_sig"))
            .build(&get_api_secret())
    }

    pub fn push_signature(&mut self) -> &mut Self {
        self.push("api_sig", self.signature())
    }

    pub fn push_api_key(&mut self) -> &mut Self {
        self.push("api_key", get_api_key())
    }

    pub fn push_token(&mut self, token: &OAuthToken) -> &mut Self {
        self.push("token", token)
    }
}


#[cfg(test)]
mod tests {
    use super::Args;

    #[test]
    fn args() {
        let args = Args::new()
            .push("test", 1)
            .push("important_key", "abc")
            .build();
        assert!(args == "test=1&important_key=abc" || args == "important_key=abc&test=1");
    }
}
