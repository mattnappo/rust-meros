use crate::core::Compressable;
use crate::crypto::{encryption, hash};
use crate::db::{IsKey, IsValue};

/// All different errors that can be thrown within the `Shard` module.
enum ShardError {
    Invalid(crate::GeneralError),
}

/// The structure used for the identification of a shard on the meros
/// network.
pub struct ShardID(hash::Hash);

impl ShardID {
    fn from_shard(shard: &Shard) -> Result<Self, ShardError> {
        Ok(Self(hash::hash_bytes(b"temp".to_vec()))) // temp
    }
}

impl IsKey for ShardID {}

/// The structure representing a `Shard` to be stored in a node's
/// local shard database.
pub struct Shard {
    data: Vec<u8>,
    size: usize,
    timestamp: u128,

    id: ShardID,
}

impl encryption::CanEncrypt for Shard {
    fn encrypt(&self) -> Vec<u8> {}
    fn decrypt(bytes: Vec<u8>) -> Self {}
}

impl Compressable for Shard {
    fn compress(&self) -> Vec<u8> {}

    fn decompress(bytes: Vec<u8>) -> Self {}
}

impl IsValue for Shard {}
