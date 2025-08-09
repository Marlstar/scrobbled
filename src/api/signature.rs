use std::collections::BTreeMap;
use super::args::Arg;

pub struct Signature(BTreeMap<&'static str, String>);
impl From<Vec<Arg>> for Signature {
    fn from(value: Vec<Arg>) -> Self {
        Self(BTreeMap::from_iter(value))
    }
}
impl FromIterator<Arg> for Signature {
    fn from_iter<T: IntoIterator<Item = Arg>>(iter: T) -> Self {
        Self(BTreeMap::from_iter(iter))
    }
}
impl Default for Signature {
    fn default() -> Self {
        Self::new()
    }
}
impl Signature {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn push(&mut self, key: &'static str, value: String) -> &mut Self {
        self.0.insert(key, value);
        self
    }

    pub fn build(&self, secret: &str) -> String {
        let mut sig = String::new();

        // Args, sorted alphabetically
        for (k, v) in self.0.iter() {
            sig.push_str(k);
            sig.push_str(v);
        }

        // Must be at the end
        sig.push_str(secret);

        return format!("{:x}", md5::compute(&sig));
    }

    pub fn args(&self) -> Vec<Arg> {
        self.0.iter().map(|(k,v)| (*k, v.clone())).collect()
    }
}


#[cfg(test)]
mod tests {
    use super::Signature;

    #[test]
    fn signature() {
        let args = vec![
            ("first", "val1".to_string()),
            ("second", "val2".to_string()),
            ("end", "val3".to_string()),
        ];
        let sig = Signature::from(args);
        assert_eq!("e82c4c7fe5053ed899725a53122932c2", sig.build("secret"))
    }
}
