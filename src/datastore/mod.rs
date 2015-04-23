use std::path::PathBuf;

// Datastore Key
pub struct DSKey {
    path: PathBuf,
}

impl DSKey {
    pub fn new_key(s: String) -> DSKey {
        let mut p = PathBuf::new();
        p.push(&s);
        DSKey { path: p }
    }

}

pub enum DatastoreError {
    Error(String),
}

pub trait Datastore<T> {
    fn put(&mut self, key: DSKey, val: T) -> Result<Option<T>, DatastoreError>;
    fn get(&self, key: &DSKey) -> Result<Option<&T>, DatastoreError>;
    fn has(&self, key: &DSKey) -> Result<bool, DatastoreError>;
    fn delete(&mut self, key: &DSKey) -> Result<Option<T>, DatastoreError>;
    // TODO
    //fn query(&self, q: Query) -> Result<QueryResults, DatastoreError>; // TODO: define Query
}

/*
type Query = u32;
type QueryResults = bool;
*/

