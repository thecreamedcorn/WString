use std::option::Option;

type NodeRef<T> = Rc<RefCell<Node<T>>>;

struct Node<T> {
    pub data: T,
    pub last: Option<NodeRef<T>>,
    pub next: Option<NodeRef<T>>
}