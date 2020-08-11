use super::*;
use sled;

/// The different types of data that a `Database` can hold.
pub enum DbType {
    File,
    Shard,
}

/// The main database structure used to store the metadata and node
/// information for all of the files on the network, and the shards
/// on each local node.
pub struct Database<K: IsKey, V: IsValue> {
    name: String,
    database: sled::Db,

    pair: Option<(K, V)>,
}

impl<K: IsKey, V: IsValue> Database<K, V> {
    /// Create and return a new database if it does not already
    /// exist.
    pub fn new(name: &str) -> Result<Self, DatabaseError> {
        // Create the new database
        let database = sled::open(format!("{}/{}", ROOT, name))
            .map_err(|e| DatabaseError::Internal(e))?;

        Ok(Self {
            name: name.to_string(),
            database,

            pair: None,
        })
    }

    /// Insert a record into the database.
    fn insert(&mut self, k: &K, v: &V) -> Result<(), DatabaseError> {
        self.database
            .insert("test", "testing")
            .map_err(|e| DatabaseError::Internal(e));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::file::{File, FileID};
    use std::path::Path;

    #[test]
    fn test_insert() {
        let mut db = Database::<FileID, File>::new("test_db").unwrap();
        let file = &File::new(Path::new("./testfile.txt")).unwrap();

        db.insert(&file.id, &file).unwrap();
    }
}
