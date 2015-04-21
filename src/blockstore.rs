use std::path::Path;

use super::block::Block;

struct DSKey {
    path: Path,
}

impl DSKey {
    pub fn new_key(s: String) -> DSKey {
        DSKey { path: Path::new(&s) }
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

enum BlockstoreError {
    Error(String),
}

trait Blockstore {
	fn put(&mut self, block: Block) -> Result<Option<&DSKey>, BlockstoreError>;
	fn get(&self, key: &DSKey) -> Result<Option<&Block>, BlockstoreError>;
	fn has(&self, key: &DSKey) -> Result<bool, BlockstoreError>;
	fn delete_block(&mut self, key: &DSKey) -> Result<Option<Block>, BlockstoreError>;

    // TODO
	// AllDSKeysChan(ctx context.Context) (<-chan u.DSKey, error)
}

struct DSBS<DS>
    where DS : Datastore<Block> {
    ds: DS,
}

impl<DS> Blockstore for DSBS<DS>
    where DS : Datastore<Block> {
	fn get(&self, key: &DSKey) -> Result<Option<&Block>, BlockstoreError> {
        match self.ds.get(key) {
            // TODO
        }
    }

}

/*
func (bs *blockstore) Get(k u.Key) (*blocks.Block, error) {
	maybeData, err := bs.datastore.Get(k.DsKey())
	if err == ds.ErrNotFound {
		return nil, ErrNotFound
	}
	if err != nil {
		return nil, err
	}
	bdata, ok := maybeData.([]byte)
	if !ok {
		return nil, ValueTypeMismatch
	}

	return blocks.NewBlockWithHash(bdata, mh.Multihash(k))
}
*/
