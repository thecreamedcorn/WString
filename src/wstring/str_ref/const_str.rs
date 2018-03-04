use super::*;

pub struct ConstStr {
    string: &'static str
}

impl ConstStr {
    pub fn new(string: &'static str) -> ConstStr {
        ConstStr { string }
    }
}

impl RawStr for ConstStr {
    fn get_bytes(&self) -> &str {
        self.string
    }
}