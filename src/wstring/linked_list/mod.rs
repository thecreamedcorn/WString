mod node;

use std::option::Option;
use node;

struct MyLinkedList<T> {
    head: NodeRef<T>,
    tail: NodeRef<T>
}

impl<T> MyLinkedList<T> {
    pub fn new() -> MyLinkedList<T> {
        MyLinkedList {
            head: Option::None,
            tail: Option::None
        }
    }

    pub fn push(value: T) {
        let node = Option::Some(Rc::new(RefCell::new(value)));
        match tail {
            Some(last) => {
                last.next = node;
                node.last
            },
            None => {

            }
        }

    }
}