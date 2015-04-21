use std::rc::Rc;
use super::DecodedMultiHash;

struct Node<'a> {
    links: Vec<Edge<'a>>,
    data: Vec<u8>,
}

struct Edge<'a> {
    // "unique per object"??
    name: String,

    size: u64,

    target_hash: DecodedMultiHash<'a>,

    node: Rc<Node<'a>>
}
