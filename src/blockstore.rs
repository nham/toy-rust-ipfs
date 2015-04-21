use super::block::Block;
use super::datastore::{Datastore, DSKey, DatastoreError};

trait Blockstore {
	fn put(&mut self, block: Block) -> Result<Option<Block>, DatastoreError>;
	fn get(&self, key: &DSKey) -> Result<Option<&Block>, DatastoreError>;
	fn has(&self, key: &DSKey) -> Result<bool, DatastoreError>;
	fn delete_block(&mut self, key: &DSKey) -> Result<Option<Block>, DatastoreError>;

    // TODO
	// AllDSKeysChan(ctx context.Context) (<-chan u.DSKey, error)
}

// A Blockstore wrapping some `Datastore` of `Block`s
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

impl<DS> DSBS<DS> where DS: Datastore<Block> {
    fn new(ds: DS) -> DSBS<DS> {
        DSBS { ds: ds }
    }
}
