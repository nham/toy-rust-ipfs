use rust_multihash::Multihash;
use util;

pub struct Block {
    multihash: Multihash,
    data: Vec<u8>,
}

impl Block {
    pub fn new(data: Vec<u8>) -> Block {
        Block {
            multihash: util::hash(&data[..]),
            data: data,
        }
    }

    pub fn get_multihash(&self) -> &Multihash {
        &self.multihash
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data[..]
    }

    pub fn clone_multihash(&self) -> Multihash {
        self.multihash.clone()
    }
}
