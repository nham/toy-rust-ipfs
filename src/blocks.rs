use multihash;
use util;

pub struct Block {
    multihash: multihash::Multihash,
    data: Vec<u8>,
}

impl Block {
    pub fn new(data: Vec<u8>) -> Block {
        Block {
            multihash: util::hash(&data[..]),
            data: data,
        }
    }
}
