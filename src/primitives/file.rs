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

use libp2p::PeerId;

use super::shard::*;
use crate::{crypto::hash, CanSerialize};

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
/// of the file. Rather, that data is stored among the nodes described in the
/// `shards` field.
#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    /// The name of the file
    pub filename: String,

    /// A hash of the file name, bytes, and an additional salt (timestamp)
    pub id: FileID,

    /// The date of creation
    pub creation_date: u128,

    /// A checksum of the bytes of the file
    checksum: u32,

    /// Ed25519 digital signature of the entire file struct. When calculated,
    /// this field is all 0s.
    signature: Vec<u8>,

    /// The original owner of the file.
    owner: PeerId,

    /// The configuration of the shards.
    pub shard_config: ShardConfig,

    // The locations of the shards on the network
    shards: Vec<PeerId>, // For scalability: Make this a ShardID-Vec<PeerId> map
}

impl File {
    /// Abstraction method to generate the metadata of a `File`.
    /// This method does not distribute a file over the meros network.
    /// However, it does prepare the file for distribution by pre-calculating
    /// the shards, but not assigning them to any nodes yet. The network will
    /// handle that. `new` returns a File struct (which is stored in the main
    /// Kad-dht), and the actual sharded data (the Vec<Shard>).
    ///
    /// # Arguments
    /// * `path` - the path of the file to read from on the disk
    /// * `options` - information about how the data should be sharded
    pub fn new(
        path: &path::Path,
        options: ShardingOptions,
    ) -> Result<(Self, Vec<Shard>), Box<dyn Error>> {
        // Read the file from the disk to generate validation metadata
        let mut fd = fs::File::open(path)?;
        let mut file_data = Vec::new(); // The contents of the file
        fd.read_to_end(&mut file_data)?;

        let filename = path.file_name()?.to_str()?;

        // Generate a file id and get the time of hashing
        let (file_id, hash_date) = FileID::new(filename, &file_data)?;

        // Calculate the shards
        let (shards, config) = Shard::shard(file_data, options)?;
        base_file.shard_config = Some(config);

        // Init the shard data into the database
        let mut internal_shards = HashMap::new(); // The shard data stored in the dht
        for i in 0..new_shards.len() {
            internal_shards.insert(new_shards[i].id.clone(), None);
        }
        base_file.shards = Some(internal_shards); // For the struct

        shards = Some(new_shards); // For returning

        // Construct the file
        let file = Self {
            filename: filename.to_string(),
            id: file_id,
            creation_date: hash_date,
            checksum: {
                let mut hasher = Hasher::new();
                hasher.update(&file_data);
                hasher.finalize()
            },
            signature: {

            }
            owner: ,
            shard_config: None,
            shards: Vec::new() // Empty because the network will handle 
        };
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
