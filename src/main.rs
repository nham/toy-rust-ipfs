extern crate rust_multihash;

pub use rust_multihash::multihash::VecMultiHash as MultiHash;
pub use rust_multihash::multihash::{DecodedMultiHash, HashFnCode};
mod dag;
mod block;
mod blockstore;
mod datastore;

fn main() {
    println!("Hello");
}
