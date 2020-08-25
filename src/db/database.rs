use super::*;
use sled;

/// The main database structure used to store the metadata and node
/// information for all of the files on the network, and the shards
/// on each local node.
pub struct Database<K: IsKey, V: IsValue> {
    pub name: String,
    database: sled::Db,

    pair: Option<(K, V)>, // A dummy
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
    fn put(&mut self, k: &K, v: &V) -> Result<(), DatabaseError> {
        // TODO: Fix error handling here
        self.database
            .insert(
                k.to_bytes().map_err(|e| DatabaseError::Serialize(e))?,
                v.to_bytes().map_err(|e| DatabaseError::Serialize(e))?,
            )
            .map_err(|e| DatabaseError::Internal(e));
        Ok(())
    }

    // /// Fetch a record from the database.
    /*
    fn fetch(&self, k: &K) -> Result<Option<V>, DatabaseError> {
        let get = self.database
            .get(k.to_bytes().map_err(|e| DatabaseError::Serialize(e))?)
            .map_err(|e| DatabaseError::Internal(e));

        return match get {
            Some()
        }
    }
    */
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::file::{File, FileID};
    use std::path::Path;

    #[test]
    fn test_put() {
        let mut db = Database::<FileID, File>::new("test_db").unwrap();
        let (file, _) =
            &File::new(Path::new("./testfile.txt"), None).unwrap();

        db.put(&file.id, &file).unwrap();
    }
}
