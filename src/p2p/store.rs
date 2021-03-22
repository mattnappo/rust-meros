use crate::common::DATADIR;
use crate::primitives::{file, shard};
use sled;
use std::error::Error;

/// A node's local storage of shards. This is essentially just a
/// map from FileID to Vec<Shard>
pub struct ShardStore(sled::Db);

impl ShardStore {
    /// Load the database at `name` if it exists, create it if it doesn't.
    pub fn new(name: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self(sled::open(format!(
            "{}/{}/{}/{}",
            DATADIR, "identities", name, "shard_store"
        ))?))
    }

    /// Store an entire vec of shards.
    fn put(
        &mut self,
        flie_id: &file::FileID,
        shards: &Vec<shard::Shard>,
    ) -> Result<(), Box<dyn Error>> {
        self.0.insert(file_id.to_bytes()?, shards.to_bytes()?)
    }

    /// Fetch a record from the database.
    fn fetch(
        &self,
        file_id: &file::FileID,
    ) -> Result<Option<Vec<Shard>>, Box<dyn Error>> {
        let get = self
            .database
            .get(k.to_bytes().map_err(|e| DatabaseError::Serialize(e))?)
            .map_err(|e| DatabaseError::Internal(e));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::file::{File, FileID};
    use std::path::Path;

    #[test]
    fn test_put() {
        let mut store = ShardStore::new("test_db").unwrap();
        let (file, _) = &File::new(Path::new("./testfile.txt"), None).unwrap();

        db.put(&file.id, &file).unwrap();
    }
}
