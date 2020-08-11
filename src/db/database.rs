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
pub struct Database {
    name: String,
    database: sled::Db,
    db_type: DbType,
}

impl Database {
    /// Create and return a new database if it does not already
    /// exist.
    pub fn new(
        name: &str,
        db_type: DbType,
    ) -> Result<Self, DatabaseError> {
        // Determine the string location for the database
        let loc = match &db_type {
            File => "file",
            Shard => "shard",
        };

        // Create the new database
        let database =
            sled::open(format!("{}/{}/{}", super::ROOT, loc, name))
                .map_err(|e| super::DatabaseError::Internal(e))?;

        Ok(Self {
            name: name.to_string(),
            database,
            db_type,
        })
    }

    // Insert a record into the database.
    // fn insert(&mut self, key: K, val: V) -> DatabaseError {

    //}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_db() {
        Database::new("test_db", DbType::File).unwrap();
    }
}
