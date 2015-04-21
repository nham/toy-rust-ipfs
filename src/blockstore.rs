use std::path::PathBuf;

use super::block::Block;

// Datastore Key
pub struct DSKey {
    path: PathBuf,
}

impl DSKey {
    pub fn new_key(s: Vec<u8>) -> DSKey {
        let mut p = PathBuf::new();
        p.push(&s);
        DSKey { path: p }
    }

}

enum DatastoreError {
    Error(String),
}

trait Datastore<T> {
    fn put(&mut self, key: DSKey, val: T) -> Result<Option<T>, DatastoreError>;
    fn get(&self, key: &DSKey) -> Result<Option<&T>, DatastoreError>;
    fn has(&self, key: &DSKey) -> Result<bool, DatastoreError>;
    fn delete(&mut self, key: &DSKey) -> Result<Option<T>, DatastoreError>;
    fn query(&self, q: Query) -> Result<QueryResults, DatastoreError>; // TODO: define Query
}

type Query = u32;
type QueryResults = bool;

trait Blockstore {
	fn put(&mut self, block: Block) -> Result<Option<Block>, DatastoreError>;
	fn get(&self, key: &DSKey) -> Result<Option<&Block>, DatastoreError>;
	fn has(&self, key: &DSKey) -> Result<bool, DatastoreError>;
	fn delete_block(&mut self, key: &DSKey) -> Result<Option<Block>, DatastoreError>;

    // TODO
	// AllDSKeysChan(ctx context.Context) (<-chan u.DSKey, error)
}

struct DSBS<DS>
    where DS : Datastore<Block> {
    ds: DS,
}

impl<DS> Blockstore for DSBS<DS>
    where DS : Datastore<Block> {

	fn get(&self, key: &DSKey) -> Result<Option<&Block>, DatastoreError> {
        self.ds.get(key)
    }

    fn has(&self, key: &DSKey) -> Result<bool, DatastoreError> {
        self.ds.has(key)
    }


	fn put(&mut self, block: Block) -> Result<Option<Block>, DatastoreError> {
        // TODO: review go-ipfs impl, in blockstore.go
        self.ds.put(block.key(), block)
    }

	fn delete_block(&mut self, key: &DSKey) -> Result<Option<Block>, DatastoreError> {
        self.ds.delete(key)

    }

}
