use crate::core::Compressable;
use crate::crypto::{encryption, hash};
use crate::db::{IsKey, IsValue};

/// All of the errors that a `Shard` method could throw.
enum ShardError {
    SerializeError(bincode::Error),
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

/*
impl encryption::CanEncrypt<Self> for Shard {
    fn encrypt(&self) -> Result<Vec<u8>, ShardError> {
        let bytes =
            self::to_bytes().map_err(|e| ShardError::SerializeError(e))?;
    }
    fn decrypt(bytes: Vec<u8>) -> Self {}
}
*/

/*
impl Compressable for Shard {
    fn compress(&self) -> Vec<u8> {}
    fn decompress(bytes: Vec<u8>) -> Self {}
}
*/

impl crate::CanSerialize for Shard {
    fn to_bytes(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)?
    }

    fn from_bytes(bytes: Vec<u8>) -> bincode::Result<Self> {
        bincode::deserialize(&bytes[..])?
    }
}

impl IsValue for Shard {}
