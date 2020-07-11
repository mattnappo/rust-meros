use std::fs::File;
use std::io::prelude::*;

use crate::db::shard_db;

/// The structure used for the identification of a file on the meros
/// network. It is a hash calculated in a specific way, as described in
/// the `File` implementation.
type FileID = Hash;

/// The structure representing a file in the main meros dht. The actual
/// content of the file is not included in this structure. The actual
/// data of the file is stored at the nodes described in the `File`'s
/// `shard_db` field.
pub struct File<'a> {
    pub filename: &'a str,
    pub fileID: FileID,
    shard_db: database::Database<Shard>,
}

impl File<'a> {
    /// Abstraction method to generate the metadata of a `File`.
    /// This method does not distribute a file over the meros network.
    /// However, it does prepare the file for sharding by pre-calculating
    /// the shards and assigning them to null nodes (temporarily).
    fn new(path: &str) -> Self {}
}

impl Hashable for File {
    fn hash(&self) -> Hash {}
}
