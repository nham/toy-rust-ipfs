extern crate rust_multihash;

pub use rust_multihash::multihash::VecMultiHash as MultiHash;
pub use rust_multihash::multihash::{DecodedMultiHash, HashFnType};
mod dag;
mod block;
mod blockstore;
mod datastore;
pub mod util;

fn main() {
    let vmh = util::hash(b"ABC");
    println!("Hello");
}
