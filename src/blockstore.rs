use block;
use rust_multihash::Multihash;

use std::collections::HashMap;

struct Blockstore {
    store: HashMap<Multihash, block::Block>,
}

impl Blockstore {
    pub fn new() -> Blockstore {
        Blockstore { store: HashMap::new() }
    }

    pub fn delete(&mut self, mh: &Multihash) {
        self.store.remove(mh);
    }

    fn has(&self, mh: &Multihash) -> bool {
        self.store.contains_key(mh)
    }

    fn get<'a>(&'a self, mh: &Multihash) -> Option<&'a block::Block> {
        self.store.get(mh)
    }

    fn put(&mut self, block: block::Block) {
        self.store.insert(block.clone_multihash(), block);
    }
}
