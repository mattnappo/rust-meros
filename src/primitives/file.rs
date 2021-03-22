use super::shard::*;
use crate::crypto;
use crate::GeneralError;
use crate::{crypto::hash, CanSerialize};
use crc32fast::Hasher;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::{
    cmp::PartialEq,
    error::Error,
    fs,
    hash::Hash,
    io::prelude::*,
    path,
    time::{SystemTime, SystemTimeError, UNIX_EPOCH},
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

    /// Check that this FileID matches that of the information given.
    pub fn matches(&self, filename: &str, bytes: &Vec<u8>, time: u128) -> bool {
        let a = [filename.as_bytes(), &bytes[..], time.to_string().as_bytes()]
            .concat()
            .to_vec();

        true
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

/// The byte representation of a libp2p::PeerId. This alias exists for readability.
type PeerIdSerial = Vec<u8>;

/// The structure representing a file on the meros network. This structure
/// contains valuable information about a file, but does not contain the data
/// of the file. Rather, that data is stored among the nodes described in the
/// `shards` field.
#[derive(Serialize, Deserialize)]
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
    /// this field is empty.
    signature: Vec<u8>,

    /// The original owner of the file.
    owner: PeerIdSerial,

    /// The configuration of the shards.
    pub shard_config: ShardConfig,

    // The locations of the shards on the network
    shards: Vec<PeerIdSerial>, // For scalability: Make this a ShardID-Vec<PeerId> map
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
    /// * `config` - information about how the data should be sharded
    /// * `priv_key` - the private key of the owner of the file
    pub fn new(
        path: &path::Path,
        config: ShardConfig,
        priv_key: &ecies_ed25519::SecretKey,
    ) -> Result<(Self, Vec<Shard>), Box<dyn Error>> {
        // Read the file from the disk to generate validation metadata
        let mut fd = fs::File::open(path)?;
        let mut file_data = Vec::new(); // The contents of the file
        fd.read_to_end(&mut file_data)?;

        let filename = match path.file_name() {
            Some(p) => match p.to_str() {
                Some(s) => s,
                None => return Err(Box::new(GeneralError::new("invalid filename"))),
            },
            None => return Err(Box::new(GeneralError::new("invalid path"))),
        };

        // Generate a file id and get the time of hashing
        let (file_id, hash_date) = FileID::new(filename, &file_data)?;

        // Calculate the shards
        let shards = Shard::shard(file_data, &mut config)?;

        // Construct the libp2p keypair
        let pub_key = ecies_ed25519::PublicKey::from_secret(priv_key);
        let keypair = crypto::ecies_to_libp2p(priv_key, &pub_key);

        // Construct the file
        let mut file = Self {
            filename: filename.to_string(),
            id: file_id,
            creation_date: hash_date,
            checksum: {
                let mut hasher = Hasher::new();
                hasher.update(&file_data);
                hasher.finalize()
            },
            signature: Vec::new(), // Temporary so that the entire file can be signed
            owner: PeerId::from_public_key(keypair.public()).into_bytes(),
            shard_config: config,
            shards: Vec::new(), // Empty because the network will handle this part
        };

        // Calc digital signature of the file and the file bytes
        let sig_data = [&file.to_bytes()?[..], &file_data[..]].concat().to_vec();
        file.signature = keypair.sign(&sig_data)?;

        Ok((file, shards))
    }

    /// Check that a file is valid against some shards.
    fn is_valid(
        &self,
        shards: &Vec<Shard>,
        priv_key: Option<&ecies_ed25519::SecretKey>,
    ) -> bool {
        // Reconstruct the shards
        let data = match Shard::reconstruct(shards, &self.shard_config, priv_key) {
            Ok(d) => d,
            Err(e) => {
                println!("invalid file/shard pair: {:?}", e);
                return false;
            }
        };

        // Check the checksum
        let checksum = {
            let mut hasher = Hasher::new();
            hasher.update(&data);
            hasher.finalize()
        } == self.checksum;

        // Check the file id
        let file_id =
            self.id
                .matches(self.filename.as_str(), &data, self.creation_date);

        // Check the signature
        let libp2p_pk = crypto::ecies_pub_to_libp2p(&self.shard_config.pub_key); // Convert key
        let self_bytes = match self.to_bytes() {
            // Serialize self
            Ok(b) => b,
            Err(e) => {
                println!("could not serialize file: {:?}", e);
                return false;
            }
        };
        let sig_data = [&self_bytes, &data[..]].concat().to_vec(); // The data to check
        let signature = libp2p_pk.verify(&sig_data, &self.signature); // Verify the sig

        checksum && file_id && signature
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
            Some(ShardConfig {
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
