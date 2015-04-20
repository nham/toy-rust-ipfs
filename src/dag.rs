use std::rc::Rc;
use super::MultiHash;

struct Node<'a> {
    links: Vec<Edge<'a>>,
    data: Vec<u8>,
}

struct Edge<'a> {
    // "unique per object"??
    name: String,

    size: u64,

    target_hash: MultiHash<'a>,

    node: Rc<Node<'a>>
}
