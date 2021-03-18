use crc32fast::Hasher;
use serde::{Deserialize, Serialize};
use std::{
    cmp::PartialEq,
    collections::HashMap,
    fs,
    hash::Hash,
    io::prelude::*,
    path,
    time::{SystemTime, SystemTimeError, UNIX_EPOCH},
};

use super::shard::*;
use crate::{
    crypto::hash,
    db::{IsKey, IsValue},
    CanSerialize,
};

/// The structure used for the identification of a file on the meros
/// network.
#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct FileID {
    id: hash::Hash,
}

impl FileID {
    pub fn new(
        filename: &str,
        bytes: &Vec<u8>,
    ) -> Result<(Self, u128), SystemTimeError> {
        let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as u128;

        let data = [filename.as_bytes(), &bytes[..], time.to_string().as_bytes()]
            .concat()
            .to_vec();
        Ok((
            Self {
                id: hash::hash_bytes(data),
            },
            time,
        ))
    }
}

impl PartialEq for FileID {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl IsKey for FileID {}
impl CanSerialize for FileID {
    type S = Self;
    fn to_bytes(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)
    }
    fn from_bytes(bytes: Vec<u8>) -> bincode::Result<Self::S> {
        bincode::deserialize(&bytes[..])
    }
}

/// All possible errors that could be returned from `File`'s methods.
#[derive(Debug)]
pub enum FileError {
    IO(std::io::Error),
    InvalidFilepath(crate::GeneralError),
    SystemTimeError(SystemTimeError),
    ShardError(ShardError),
}

/// The structure representing a file on the meros network. This structure
/// contains valuable information about a file, but does not contain the data
/// of the file. Rather, that is stored among the nodes described in the
/// `shards` field.
#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    // Not everything needs to be public TODO
    pub filename: String,
    pub id: FileID,
    pub creation_date: u128,
    pub checksum: u32, // A checksum of just the bytes of the file
    // signature: DigitalSignature, // mock type, tbi TODO (to be implemented)
    pub shard_config: Option<ShardConfig>,

    // The locations of  the shards on the network
    //pub shards: Option<HashMap<ShardID, Option<PeerId>>>,
    pub shards: Option<HashMap<ShardID, Option<u32>>>, // temporary
}

impl File {
    /// Abstraction method to generate the metadata of a `File`.
    /// This method does not distribute a file over the meros network.
    /// However, it does prepare the file for sharding by pre-calculating
    /// the shards and assigning them to null nodes (temporarily).
    pub fn new(
        path: &path::Path,
        sharding_options: Option<ShardingOptions>,
    ) -> Result<(Self, Option<Vec<Shard>>), FileError> {
        // Read the file from the disk to generate validation metadata
        let mut fd = fs::File::open(path).map_err(|e| FileError::IO(e))?;
        let mut buf = Vec::new(); // The contents of the file
        fd.read_to_end(&mut buf).map_err(|e| FileError::IO(e))?;

        // Get the name of the file (clean this up somehow TODO)
        let invalid_path =
            Err(FileError::InvalidFilepath(crate::GeneralError::new(
                format!("{} is an invalid filepath", path.display()).as_str(),
            )));
        let filename = match path.file_name() {
            Some(name) => match name.to_str() {
                Some(s) => s,
                None => return invalid_path,
            },
            None => return invalid_path,
        };

        // Generate a file id and get the time
        let (file_id, hash_date) = FileID::new(filename, &buf)
            .map_err(|e| FileError::SystemTimeError(e))?;

        // Construct the file
        let mut base_file = Self {
            filename: filename.to_string(),
            id: file_id,
            creation_date: hash_date,
            checksum: {
                let mut hasher = Hasher::new();
                hasher.update(&buf);
                hasher.finalize()
            },
            shard_config: None,
            shards: None,
        };

        let mut shards: Option<Vec<Shard>> = None;

        // Shard if there are sharding options
        if let Some(options) = sharding_options {
            let (new_shards, config) = Shard::shard(buf, options)
                .map_err(|e| FileError::ShardError(e))?;
            base_file.shard_config = Some(config);

            // Init the shard data into the database
            let mut internal_shards = HashMap::new(); // The shard data stored in the dht
            for i in 0..new_shards.len() {
                internal_shards.insert(new_shards[i].id.clone(), None);
            }
            base_file.shards = Some(internal_shards); // For the struct

            shards = Some(new_shards); // For returning
        }

        Ok((base_file, shards))
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.filename == other.filename && self.id == other.id
    }
}

impl super::Hashable for File {
    fn hash(&self) -> hash::Hash {
        [0 as u8; 32] // temp
    }
}

impl IsValue for File {}

impl CanSerialize for File {
    type S = Self;
    fn to_bytes(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)
    }
    fn from_bytes(bytes: Vec<u8>) -> bincode::Result<Self::S> {
        bincode::deserialize(&bytes[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::encryption;
    use std::path::Path;

    #[test]
    fn test_new_file() {
        File::new(Path::new("testfile.txt"), None).unwrap();
    }

    #[test]
    fn test_new_file_with_sharding() {
        let (_, public) = encryption::gen_keypair("testkey", false).unwrap();

        let (file, shards) = File::new(
            Path::new("testfile.txt"),
            Some(ShardingOptions {
                shard_count: 10,
                public_key: Some(public),
                private_key: None,
                compress: false,
            }),
        )
        .unwrap();

        let shards = shards.unwrap();
        let internal_shards = file.shards.unwrap();

        println!("Map of internal shards: {:?}", internal_shards);
        println!("the shards: {:?}", shards);
        for i in 0..shards.len() {
            match internal_shards.get(&shards[i].id) {
                Some(k) => continue,
                None => {
                    panic!("shards are out of order or incorrectly constructed")
                }
            }
        }
    }

    #[test]
    fn test_to_bytes() {
        let file = File::new(Path::new("testfile.txt"), None).unwrap();
        let bytes = file.0.to_bytes().unwrap();
        println!("bytes: {:?}", bytes);
    }

    #[test]
    fn test_from_bytes() {
        let file = File::new(Path::new("testfile.txt"), None).unwrap();
        let serialized = file.0.to_bytes().unwrap();

        let deserialized = File::from_bytes(serialized).unwrap();
        println!("deserialized file: {:?}", deserialized);

        // assert_eq!(file, deserialized);
    }
}
