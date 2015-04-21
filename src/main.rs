extern crate rust_multihash;

pub use rust_multihash::multihash::VecMultiHash as MultiHash;
pub use rust_multihash::multihash::{DecodedMultiHash, HashFnCode};
mod dag;
mod block;
mod blockstore;

fn main() {
    println!("Hello");
}
