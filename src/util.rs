use rust_multihash::multihash;
use super::MultiHash;

pub fn hash<'a>(data: &'a[u8]) -> MultiHash {
    multihash::hash(data, multihash::HashFnCode::SHA1)
}
