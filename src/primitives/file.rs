use crate::db::shard_db;

/// The structure representing a file in the main meros dht. The actual
/// content of the file is not included in this structure. The actual
/// data of the file is stored at the nodes described in the `File`'s
/// `shard_db` field.
pub struct File {
    pub filename: str,
    pub fileID: Vec<u8>,
    shard_db: shard_db::ShardDB,
}
