use crate::{
    crypto::{encryption, hash, hash::HASH_SIZE},
    CanSerialize, GeneralError,
};

use ecies_ed25519::{PublicKey, SecretKey};
use math::round::floor;
use serde::{Deserialize, Serialize};
use std::{
    clone::Clone,
    cmp::PartialEq,
    error::Error,
    fmt,
    hash::Hash,
    time::{SystemTime, UNIX_EPOCH},
};

/// The structure used for the identification of a shard on the meros
/// network.
#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct ShardID {
    id: hash::Hash,
}

impl ShardID {
    // Calculate a ShardID of the data in a shard.
    pub fn new(data: &Vec<u8>) -> Result<(Self, u128), Box<dyn Error>> {
        let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as u128;

        let data = [&data[..], time.to_string().as_bytes()].concat().to_vec();
        Ok((
            Self {
                id: hash::hash_bytes(data),
            },
            time,
        ))
    }

    /// Construct a ShardID from the bytes of a ShardID. This does not
    /// guarantee that the ShardID is a valid ShardID.
    pub fn from_bytes(bytes: [u8; HASH_SIZE]) -> Self {
        ShardID { id: bytes }
    }

    /// Check that this ShardID matches that of the data and timestamp given.
    pub fn matches(&self, data: &Vec<u8>, time: u128) -> bool {
        &ShardID::from_bytes(hash::hash_bytes(
            [&data[..], time.to_string().as_bytes()].concat().to_vec(),
        )) == self
    }
}

impl Clone for ShardID {
    fn clone(&self) -> Self {
        let mut v = [0u8; HASH_SIZE];

        for i in 0..HASH_SIZE {
            v[i] = self.id[i];
        }

        Self { id: v }
    }
}

impl PartialEq for ShardID {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ShardID {}

impl CanSerialize for ShardID {
    type S = Self;
    fn to_bytes(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)
    }
    fn from_bytes(bytes: Vec<u8>) -> bincode::Result<Self> {
        bincode::deserialize(&bytes[..])
    }
}

/// A structure used to configure how a vector of bytes is sharded.
#[derive(Serialize, Deserialize, Clone)]
pub struct ShardConfig {
    /// The number of shards.
    pub shard_count: usize, // make u16

    /// The owner's public key.
    pub pub_key: PublicKey,

    /// Whether the shard is compressed or not
    pub compress: bool,

    /// Whether the shard is encrypted or not
    pub encrypt: bool,

    /// The sizes of the shards, in order
    pub sizes: Vec<usize>,
}

impl fmt::Debug for ShardConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ShardConfig")
            .field("shard_count", &self.shard_count)
            .field("pub_key", &self.pub_key.to_bytes())
            .field("compress", &self.compress)
            .field("encrypt", &self.encrypt)
            .field("sizes", &self.sizes)
            .finish()
    }
}

impl ShardConfig {
    /// Create the default shard config (will be overwritten by file::new())
    pub fn new(n_shards: usize, pk: &PublicKey) -> Self {
        Self {
            shard_count: n_shards,
            pub_key: pk.clone(),
            compress: false,
            encrypt: false,
            sizes: Vec::new(),
        }
    }
}

impl CanSerialize for ShardConfig {
    type S = Self;
    fn to_bytes(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)
    }
    fn from_bytes(bytes: Vec<u8>) -> bincode::Result<Self> {
        bincode::deserialize(&bytes[..])
    }
}

/// The structure representing a `Shard` to be stored in a node's
/// local shard database.
#[derive(Serialize, Deserialize, Debug, Hash, Clone)]
pub struct Shard {
    // A unique ID, used for identification on the network
    pub id: ShardID,

    /// The actual data of the shard
    pub data: Vec<u8>,

    // The size of the data in the shard
    size: usize,

    // The time at which the shard was created
    timestamp: u128,

    // The index of the shard in a larger vector of shards
    index: u32,
}

impl Shard {
    // Create a new shard
    pub fn new(data: Vec<u8>, index: u32) -> Result<Shard, Box<dyn Error>> {
        let (id, timestamp) = ShardID::new(&data)?;

        Ok(Shard {
            size: data.len(),
            data,
            timestamp,
            index,
            id,
        })
    }

    // Run various checks to determine if a shard is valid.
    pub fn is_valid(&self) -> bool {
        // Check the size and the fileID
        self.size == self.data.len() && self.id.matches(&self.data, self.timestamp)
    }

    /// Given some bytes, split the bytes and return a vector of `Shard`s.
    pub fn shard(
        bytes: &Vec<u8>,
        config: ShardConfig,
    ) -> Result<(Vec<Shard>, ShardConfig), Box<dyn Error>> {
        // Encrypt the bytes
        let mut b = bytes;
        let mut a: Vec<u8> = Vec::new();
        if config.encrypt {
            a = encryption::encrypt_bytes(&config.pub_key, &b)?;
        }
        // Clean this up, very hacky
        if a.len() > 0 {
            b = &a;
        }

        // Shard the bytes
        let sizes = calculate_shard_sizes(b.len(), config.shard_count)?;
        let shards = split_bytes(&b, &sizes)?;

        // Update the config
        let mut new_config = config.clone();
        new_config.shard_count = sizes.len();
        new_config.sizes = sizes;

        Ok((shards, new_config))
    }

    /// The inverse operation of `shard`. Extracts and reconstructs the bytes
    /// stored inside the given shards.
    pub fn reconstruct(
        shards: &Vec<Shard>, // Just bytes for now for the same debugging purposes
        config: &ShardConfig,
        private_key: Option<&SecretKey>,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        // Reconstruct
        let mut data: Vec<u8> = Vec::new();
        let mut counter = 0;
        for shard in shards.iter() {
            // For each shard
            // Validate the shard
            if shard.is_valid() && shard.index == counter {
                // Extract the shard data
                for byte in shard.data.iter() {
                    data.push(*byte);
                }
            } else {
                return Err(Box::new(GeneralError::new(
                    "invalid shard; cannot use it to reconstruct",
                )));
            }
            counter += 1;
        }

        // Decrypt if encrypted
        if config.encrypt {
            return match private_key {
                Some(key) => Ok(encryption::decrypt_bytes(&key, &data).unwrap()),
                None => Err(Box::new(GeneralError::new(
                    "private key not given, cannot decrypt shard data",
                ))),
            };
        }
        Ok(data)
    }
}

/// Split a vector of bytes as described by the `sizes` parameter and
/// return properly distributed `Shard`s.
fn split_bytes(
    bytes: &Vec<u8>,
    sizes: &Vec<usize>,
) -> Result<Vec<Shard>, Box<dyn Error>> {
    // Validate the `sizes` vector
    if sizes.iter().sum::<usize>() != bytes.len() || sizes.contains(&0) {
        return Err(Box::new(GeneralError::new(
            format!("{:?} is not a valid vector of byte split sizes.", sizes)
                .as_str(),
        )));
    }

    let mut shards: Vec<Shard> = Vec::new();
    let mut byte_pointer = 0usize;

    // Iterate through each size and create a shard with that data
    for i in 0..sizes.len() {
        let size = sizes[i];
        let sliced_bytes = &bytes[byte_pointer..size + byte_pointer];

        shards.push(Shard::new(sliced_bytes.to_vec(), i as u32)?);
        byte_pointer += size;
    }

    Ok(shards)
}

/// Calculate a vector of recommended shard data sizes for a given
/// length of data and number of partitions. This algorithm calculates
/// the most equal distribution of shard sizes.
fn calculate_shard_sizes(
    n_bytes: usize,
    n_partitions: usize,
) -> Result<Vec<usize>, Box<dyn Error>> {
    // Validate the inputs
    if n_bytes == 0 || n_partitions == 0 || n_partitions > n_bytes {
        println!("n bytes: {}, n partitions: {}", n_bytes, n_partitions);
        return Err(Box::new(GeneralError::new(
            "invalid parameters to calculate shard sizes",
        )));
    }

    // The average byte size of each partition
    let avg = floor((n_bytes / n_partitions) as f64, 0) as usize;

    // The amount of bytes left over
    let extra = n_bytes % n_partitions;

    let mut sizes: Vec<usize> = vec![avg; n_partitions];
    let len = sizes.len();
    sizes[len - 1] += extra;

    // Before returning, just make sure that everything went well
    if sizes.iter().sum::<usize>() != n_bytes {
        return Err(Box::new(GeneralError::new(
            "unable to calculate shard sizes",
        )));
    }
    Ok(sizes)
}

impl PartialEq for Shard {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
            && self.size == other.size
            && self.timestamp == other.timestamp
            && self.id == other.id
    }
}
/*
impl Compressable for Shard { fn compress(&self) -> Vec<u8> {}
    fn decompress(bytes: Vec<u8>) -> Self {}
}
*/

impl CanSerialize for Shard {
    type S = Self;
    fn to_bytes(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)
    }
    fn from_bytes(bytes: Vec<u8>) -> bincode::Result<Self> {
        bincode::deserialize(&bytes[..])
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::CanSerialize;

    #[test]
    fn test_to_bytes() {
        let shard = Shard::new(vec![1u8, 10u8], 1).unwrap();
        assert_eq!(shard.size, 2);

        println!("shard: {:?}", shard);
        shard.to_bytes().unwrap();
    }

    #[test]
    fn test_from_bytes() {
        let serialized = Shard::new(vec![1u8, 10u8], 1).unwrap().to_bytes().unwrap();
        /*
                let extra_bytes: &[u8] = &[
                    2u8, 5u8, 2u8, 5u8, 2u8, 5u8, 2u8, 5u8, 2u8, 5u8, 2u8, 5u8,
                    2u8, 5u8, 2u8, 5u8,
                ];
                let serialized = [extra_bytes, &serialized[..]].concat();
        */
        let deserialized = Shard::from_bytes(serialized).unwrap();
        println!("deserialized shard: {:?}", deserialized);
    }

    #[test]
    fn test_split_bytes() {
        // Test 1
        let bytes: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7];
        let sizes: Vec<usize> = vec![1, 2, 1, 1, 2];

        let shards = split_bytes(&bytes, &sizes).unwrap();

        assert_eq!(shards[0].data, vec![1u8]);
        assert_eq!(shards[1].data, vec![2u8, 3u8]);
        assert_eq!(shards[2].data, vec![4u8]);
        assert_eq!(shards[3].data, vec![5u8]);
        assert_eq!(shards[4].data, vec![6u8, 7u8]);

        // Test 2
        let bytes: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 12, 9, 17, 15, 7];
        let sizes: Vec<usize> = vec![9, 3, 1]; // 13 total

        let shards = split_bytes(&bytes, &sizes).unwrap();

        assert_eq!(shards[0].data, vec![1u8, 2, 3, 4, 5, 6, 7, 8, 12]);
        assert_eq!(shards[1].data, vec![9u8, 17, 15]);
        assert_eq!(shards[2].data, vec![7u8]);
    }

    #[test]
    fn test_calc_shard_sizes() {
        let t1 = calculate_shard_sizes(10, 3).unwrap();
        let t2 = calculate_shard_sizes(12312238, 27).unwrap();
        let t3 = calculate_shard_sizes(0xFF * 2, 19).unwrap();
        println!("t1: {:?}\nt2: {:?}\nt3: {:?}", t1, t2, t3);
    }

    fn test_shard_case(my_bytes: Vec<u8>, n_shards: usize) {
        let shards = Shard::shard(
            my_bytes.clone(),
            ShardConfig {
                shard_count: n_shards,
                public_key: None,
                private_key: None,
                compress: false,
            },
        )
        .unwrap();

        // Piece the data from the shards back together
        let mut data: Vec<u8> = Vec::new();
        for shard in shards.0.iter() {
            for byte in shard.data.iter() {
                data.push(*byte);
            }
        }
        assert_eq!(my_bytes, data);
    }

    // Test `Shard::shard` without encryption
    #[test]
    fn test_shard_no_encrypt() {
        // Simple test case
        let mut b: Vec<u8> = Vec::new();
        for byte in 0..0xFF {
            b.push(byte);
            b.push(byte);
        }
        test_shard_case(b, 19usize);

        // Do some more automated testing
        let mut rng = rand::thread_rng();
        for _i in 0..10 {
            // Generate a lot of bytes
            let mut b: Vec<u8> = Vec::new();
            for _i in 0..rng.gen_range(1, 100_000) {
                b.push(rng.gen_range(0, 0xFF) as u8);
            }
            let len = b.len();
            test_shard_case(b, rng.gen_range(1, len));
        }
    }

    #[test]
    fn test_reconstruct_no_encrypt() {
        let mut b: Vec<u8> = Vec::new();
        for byte in 0..0xFF {
            b.push(byte);
        }

        let shard_count = 6;

        let (shards, _) = Shard::shard(
            b.clone(),
            ShardConfig {
                shard_count,
                public_key: None,
                private_key: None,
                compress: false,
            },
        )
        .unwrap();

        let reconstructed = Shard::reconstruct(
            &shards,
            ShardConfig {
                shard_count,
                public_key: None,
                private_key: None,
                compress: false,
            },
        )
        .unwrap();

        assert_eq!(b, reconstructed);
    }

    #[test]
    fn test_reconstruct_encrypt() {
        let mut b: Vec<u8> = Vec::new();
        for byte in 0..0xFF {
            b.push(byte);
        }

        let (pub_key, priv_key) = (
            encryption::load_pub_key(&encryption::KeyType::Public(
                "testkey".to_string(),
            ))
            .unwrap(),
            encryption::load_priv_key(&encryption::KeyType::Private(
                "testkey".to_string(),
            ))
            .unwrap(),
        );

        let sc = 11; // Whatever (shard count)

        // Shard with encryption
        let (shards, _) = Shard::shard(
            b.clone(),
            ShardConfig {
                shard_count: sc,
                // public_key: None,
                public_key: Some(pub_key),
                private_key: None,
                compress: false,
            },
        )
        .unwrap();

        // Reconstruct
        let reconstructed_b = Shard::reconstruct(
            &shards, // The shards themselves
            ShardConfig {
                shard_count: sc,
                public_key: None,
                // private_key: None,
                private_key: Some(priv_key),
                compress: false,
            },
        )
        .unwrap();

        assert_eq!(b, reconstructed_b);
    }
}
*/
