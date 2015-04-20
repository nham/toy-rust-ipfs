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


/*
 *

Thread-local reference-counted boxes, Rc<T>

Rc<T> type provides shared ownership of an immutable value. Destruction is deterministic, occurs as soon as last owner is gone.

Non-sendable because it avoids overhead of atomic reference counting, which is apparently needed for something?

A Weak<T> pointer, whatever that is, can be upgraded to an Rc<T> pointer.



 */
