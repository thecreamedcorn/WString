mod byte_buff;
mod const_str;

use std::rc::Rc;
use const_str::ConstStr;
use byte_buff::ByteBuffer;

pub trait RawStr {
    fn get_str(&self) -> &str;
}

pub struct StrRef {
    ptr: Rc<StrRef>,
    pub start: usize,
    pub end: usize
}

impl StrRef {
    pub fn fromLiteral(string: &'static str, start: usize, end: usize) -> StrRef {
        StrRef {
            ptr: Rc::new(ConstStr::new(string)),
            start,
            end
        }
    }

    pub fn fromStr(string: &str, start: usize, end:usize) -> StrRef {
        StrRef {
            ptr: Rc::new(ByteBuffer::new(string)),
            start,
            end
        }
    }
}