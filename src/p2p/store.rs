use crate::{
    common::DATADIR,
    primitives::{file, shard},
    CanSerialize, GeneralError,
};
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
    ) -> Result<Option<sled::IVec>, Box<dyn Error>> {
        let shards_bytes = bincode::serialize(shards)?;

        self.0
            .insert(file_id.to_bytes()?, shards_bytes)
            .map_err(|e| e.into())
    }

    /// Get all the shards attached to a file id
    fn get(
        &self,
        file_id: &file::FileID,
    ) -> Result<Option<Vec<shard::Shard>>, Box<dyn Error>> {
        let shards_bytes = self.0.get(file_id.to_bytes()?)?;
        match shards_bytes {
            Some(bytes) => {
                Ok(Some(bincode::deserialize::<Vec<shard::Shard>>(&bytes)?))
            }
            None => Err(Box::new(GeneralError::new(
                format!("no shards in shardstore for {:?}", file_id).as_str(),
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::file::File;
    use crate::primitives::shard::ShardConfig;
    use ecies_ed25519::*;
    use std::path::Path;

    fn keypair() -> (SecretKey, PublicKey) {
        let mut csprng = rand::thread_rng();
        generate_keypair(&mut csprng)
    }

    #[test]
    fn test_put_get() {
        let (sk, pk) = keypair();
        let (file, shards) =
            &File::new(Path::new("./testfile.txt"), ShardConfig::new(5, &pk), &sk)
                .unwrap();

        let mut store = ShardStore::new("test_db").unwrap();
        store.put(&file.id, &shards).unwrap();

        match store.get(&file.id).unwrap() {
            Some(s) => assert_eq!(shards, &s),
            None => panic!(),
        }
    }
}
