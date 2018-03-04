use super::*;

pub struct ByteBuff {
    string: String
}

impl ByteBuff {
    pub fn new(string: &str) -> ByteBuff {
        ByteBuff {
            string: String::from(string)
        }
    }
}

impl RawStr for ByteBuff {
    fn get_str(&self) -> &str {
        &self.string
    }
}