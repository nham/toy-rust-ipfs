use std::path::PathBuf;

use super::block::Block;

// Datastore Key
struct DSKey {
    path: PathBuf,
}

impl DSKey {
    pub fn new_key(s: String) -> DSKey {
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
	fn put(&mut self, block: Block) -> Result<Option<&DSKey>, DatastoreError>;
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
    // fn get(&self, key: &DSKey) -> Result<Option<&T>, DatastoreError>;

    fn has(&self, key: &DSKey) -> Result<bool, DatastoreError> {
        self.ds.has(key)
    }

	fn put(&mut self, block: Block) -> Result<Option<&DSKey>, DatastoreError> {
    }

	fn delete_block(&mut self, key: &DSKey) -> Result<Option<Block>, DatastoreError> {

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
