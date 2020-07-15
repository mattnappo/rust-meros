use crate::{
    core::Compressable,
    crypto::hash,
    db::{IsKey, IsValue},
};
use serde::{Deserialize, Serialize};

/// All of the errors that a `Shard` method could throw.
enum ShardError {
    SerializeError(bincode::Error),
}

/// The structure used for the identification of a shard on the meros
/// network.
#[derive(Serialize, Deserialize)]
pub struct ShardID(hash::Hash);

impl ShardID {
    fn from_shard(shard: &Shard) -> Result<Self, ShardError> {
        Ok(Self(hash::hash_bytes(b"temp".to_vec()))) // temp
    }
}

impl IsKey for ShardID {}

/// The structure representing a `Shard` to be stored in a node's
/// local shard database.
#[derive(Serialize, Deserialize)]
pub struct Shard {
    data: Vec<u8>,
    size: usize,
    timestamp: u128,

    id: ShardID,
}

/*
impl Compressable for Shard {
    fn compress(&self) -> Vec<u8> {}
    fn decompress(bytes: Vec<u8>) -> Self {}
}
*/

impl crate::CanSerialize for Shard {
    type S = Self;
    fn to_bytes(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)
    }
    fn from_bytes(bytes: Vec<u8>) -> bincode::Result<Self> {
        bincode::deserialize(&bytes[..])
    }
}

impl IsValue for Shard {}
