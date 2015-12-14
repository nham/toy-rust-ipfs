use rust_multihash::Multihash;
use util;

pub struct Block {
    multihash: Multihash,
    data: Vec<u8>,
}

impl Block {
    pub fn new(data: Vec<u8>) -> Self {
        Block {
            multihash: util::hash(&data[..]),
            data: data,
        }
    }

    pub fn with_hash(data: Vec<u8>, multihash: Multihash) -> Self {
        Block {
            multihash: multihash,
            data: data,
        }
    }

    pub fn get_multihash(&self) -> &Multihash {
        &self.multihash
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data[..]
    }

    pub fn take_data(self) -> Vec<u8> { self.data }

    pub fn clone_multihash(&self) -> Multihash {
        self.multihash.clone()
    }
}
