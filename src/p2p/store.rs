use crate::common::DATADIR;
use crate::primitives::{file, shard};
use crate::CanSerialize;
use sled;
use std::error::Error;

/// A node's local storage of shards. This is essentially just a
/// map from FileID to Vec<Shard>
pub struct ShardStore(sled::Db);

impl ShardStore {
    /// Load the database at `name` if it exists, create it if it doesn't
    pub fn new(name: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self(sled::open(format!(
            "{}/{}/{}/{}",
            DATADIR, "identities", name, "shard_store"
        ))?))
    }

    /// Store an entire vec of shards.
    fn put(
        &mut self,
        file_id: &file::FileID,
        shards: &Vec<shard::Shard>,
    ) -> Result<(), Box<dyn Error>> {
        let shards_bytes = shards
            .into_iter()
            .map(|s| s.to_bytes().unwrap())
            .collect::<Vec<Vec<u8>>>()
            .concat();
        println!("shard bytes: {:?}", shards_bytes);
        Ok(())
        //match self.0.insert(file_id.to_bytes()?, shards.to_bytes()?) {
        //    Ok(_) => _,
        //    Err(e) => e
        //}
    }

    /*
    /// Get all the shards attached to a file id
    fn get(
        &self,
        file_id: &file::FileID,
    ) -> Result<Option<Vec<Shard>>, Box<dyn Error>> {
        let get = self
            .database
            .get(k.to_bytes().map_err(|e| DatabaseError::Serialize(e))?)
            .map_err(|e| DatabaseError::Internal(e));
    }
    */
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::file::{File, FileID};
    use crate::primitives::shard::ShardConfig;
    use ecies_ed25519::*;
    use std::path::Path;

    fn keypair() -> (SecretKey, PublicKey) {
        let mut csprng = rand::thread_rng();
        generate_keypair(&mut csprng)
    }

    #[test]
    fn test_put() {
        let (sk, pk) = keypair();
        let (file, shards) = &File::new(
            Path::new("./testfile.txt"),
            ShardConfig::with_pubkey(&pk),
            &sk,
        )
        .unwrap();

        let mut store = ShardStore::new("test_db").unwrap();
        store.put(&file.id, &shards).unwrap();
    }
}
