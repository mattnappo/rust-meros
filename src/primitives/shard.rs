use crate::{
    core::Compressable,
    crypto::hash,
    db::{IsKey, IsValue},
    GeneralError,
};
use math::round::floor;
use serde::{Deserialize, Serialize};
use std::{
    cmp::PartialEq,
    time::{SystemTime, SystemTimeError, UNIX_EPOCH},
};

/// All of the errors that a `Shard` method could throw.
#[derive(Debug)]
pub enum ShardError {
    SerializeError(bincode::Error),
    TimestampError(SystemTimeError),
    InvalidSplitSizes(GeneralError),
    NullShardData(GeneralError),
}

/// The structure used for the identification of a shard on the meros
/// network.
#[derive(Serialize, Deserialize, Debug)]
pub struct ShardID {
    id: hash::Hash,
}

impl ShardID {
    pub fn new(data: &Vec<u8>) -> Result<(Self, u128), ShardError> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| ShardError::TimestampError(e))?
            .as_secs() as u128;

        let data =
            [&data[..], time.to_string().as_bytes()].concat().to_vec();
        Ok((
            Self {
                id: hash::hash_bytes(data),
            },
            time,
        ))
    }
}

impl PartialEq for ShardID {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl IsKey for ShardID {}

/// The structure representing a `Shard` to be stored in a node's
/// local shard database.
#[derive(Serialize, Deserialize, Debug)]
pub struct Shard {
    pub data: Vec<u8>,
    size: usize,
    timestamp: u128,
    index: u32,

    id: ShardID,
}

impl Shard {
    pub fn new(data: Vec<u8>, index: u32) -> Result<Shard, ShardError> {
        let (id, timestamp) = ShardID::new(&data)?;

        Ok(Shard {
            size: data.len(),
            data,
            timestamp,
            index,
            id,
        })
    }
}

/// Split a vector of bytes as described by the `sizes` parameter and
/// return properly distributed `Shard`s.
pub fn split_bytes(
    bytes: &Vec<u8>,
    sizes: &Vec<usize>,
) -> Result<Vec<Shard>, ShardError> {
    // Validate the `sizes` vector
    if sizes.iter().sum::<usize>() != bytes.len() || sizes.contains(&0) {
        return Err(ShardError::InvalidSplitSizes(GeneralError::new(
            format!(
                "{:?} is not a valid vector of byte split sizes.",
                sizes,
            )
            .as_str(),
        )));
    }

    let mut shards: Vec<Shard> = Vec::new();
    let mut byte_pointer = 0usize;

    // Iterate through each size and create a shard with that data
    for i in 0..sizes.len() {
        let size = sizes[i];
        let mut sliced_bytes = &bytes[byte_pointer..size + byte_pointer];

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
) -> Result<Vec<usize>, ShardError> {
    // Validate the inputs
    if n_bytes == 0 || n_partitions == 0 {
        return Err(
            ShardError::NullShardData(
                GeneralError::new(
                    "cannot calculate shard sizes due to null number of partitions or bytes"
                )
            )
        );
    }

    // The average byte size of each partition
    let avg = floor((n_bytes / n_partitions) as f64, 0) as usize;

    // The amount of bytes left over
    let extra = n_bytes % n_partitions;

    let mut sizes: Vec<usize> = vec![avg; n_partitions];
    let len = sizes.len();
    sizes[len - 1] += extra;

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
        let shard = Shard::new(vec![1u8, 10u8], 1).unwrap();
        assert_eq!(shard.size, 2);

        println!("shard: {:?}", shard);
        shard.to_bytes().unwrap();
    }

    #[test]
    fn test_from_bytes() {
        let serialized =
            Shard::new(vec![1u8, 10u8], 1).unwrap().to_bytes().unwrap();
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
        let bytes: Vec<u8> = vec![1, 2, 3, 0, 5, 6, 7];
        let sizes: Vec<usize> = vec![1, 2, 1, 1, 2];

        let shards = split_bytes(&bytes, &sizes).unwrap();

        assert_eq!(shards[0].data, vec![1u8]);
        assert_eq!(shards[1].data, vec![2u8, 3u8]);
        assert_eq!(shards[2].data, vec![4u8]);
        assert_eq!(shards[3].data, vec![5u8]);
        assert_eq!(shards[4].data, vec![6u8, 7u8]);

        // Test 2
        let bytes: Vec<u8> =
            vec![1, 2, 3, 4, 5, 6, 7, 8, 12, 9, 17, 15, 7];
        let sizes: Vec<usize> = vec![9, 3, 1]; // 13 total

        let shards = split_bytes(&bytes, &sizes).unwrap();

        assert_eq!(shards[0].data, vec![1u8, 2, 3, 4, 5, 6, 7, 8, 12]);
        assert_eq!(shards[1].data, vec![9u8, 17, 15]);
        assert_eq!(shards[2].data, vec![7u8]);
    }
}
