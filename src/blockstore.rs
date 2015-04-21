type Key = String;

enum DatastoreError {
    Error(String),
}

trait Datastore<T> {
    fn put(&mut self, key: Key, val: T) -> Result<Option<T>, DatastoreError>;
    fn get(&self, key: &Key) -> Result<Option<&T>, DatastoreError>;
    fn has(&self, key: &Key) -> Result<bool, DatastoreError>;
    fn delete(&mut self, key: &Key) -> Result<T, DatastoreError>;
    fn query(&self, q: Query) -> Result<QueryResults, DatastoreError>; // TODO: define Query
}

type Query = u32;
type QueryResults = bool;
