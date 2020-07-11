use crate::db::shard_db;

/// The structure used for the identification of a file on the meros
/// network. It is a hash calculated in a specific way, as described in
/// the `File` implementation.
type FileID = Hash;

/// The structure representing a file in the main meros dht. The actual
/// content of the file is not included in this structure. The actual
/// data of the file is stored at the nodes described in the `File`'s
/// `shard_db` field.
pub struct File {
    pub filename: str,
    pub fileID: FileID,
    shard_db: shard_db::ShardDB,
}

impl File {
    /// Abstraction method to generate the metadata of a `File`.
    /// This method does not distribute a file over the meros network.
    fn new_file_from_disk(filepath: &str) {}
    fn calc_file_id(&self) ->
}

impl Hashable for File {
    fn hash(&self) -> Hash {}
}
