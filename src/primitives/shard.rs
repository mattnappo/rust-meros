use crate::{
    core::Compressable,
    crypto::hash,
    db::{IsKey, IsValue},
};

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

/// All of the errors that a `Shard` method could throw.
#[derive(Debug)]
enum ShardError {
    SerializeError(bincode::Error),
    ShardIDError(SystemTimeError),
}

/// The structure used for the identification of a shard on the meros
/// network.
#[derive(Serialize, Deserialize, Debug)]
pub struct ShardID(hash::Hash);

impl ShardID {
    pub fn new(data: &Vec<u8>) -> Result<(Self, u128), SystemTimeError> {
        let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()
            as u128;

        let data =
            [&data[..], time.to_string().as_bytes()].concat().to_vec();
        Ok((Self(hash::hash_bytes(data)), time))
    }
}

impl IsKey for ShardID {}

/// The structure representing a `Shard` to be stored in a node's
/// local shard database.
#[derive(Serialize, Deserialize, Debug)]
pub struct Shard {
    data: Vec<u8>,
    size: usize,
    timestamp: u128,

    id: ShardID,
}

impl Shard {
    fn new(data: Vec<u8>) -> Result<Shard, ShardError> {
        let (id, timestamp) = ShardID::new(&data)
            .map_err(|e| ShardError::ShardIDError(e))?;

        Ok(Shard {
            size: data.len(),
            data,
            timestamp,
            id,
        })
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CanSerialize;

    #[test]
    fn test_to_bytes() {
        let shard = Shard::new(vec![1 as u8, 10 as u8]).unwrap();
        println!("shard: {:?}", shard);
        shard.to_bytes().unwrap();
    }
}
