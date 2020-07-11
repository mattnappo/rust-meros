use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::db::shard_db;

/// The structure used for the identification of a file on the meros
/// network. It is a hash calculated in a specific way, as described in
/// the `File` implementation.
type FileID = Hash;

/// All possible errors that could be returned from `File`'s methods.
enum FileError {
  IO(std::io::Error),
  InvalidFilepath(InvalidFilepath),
}

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
    fn new(path: path::Path) -> Result<Self, FileError> {
        let mut file = File::open(path).map_err(|e| FileError::IO(e))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).map_err(|e| FileError::IO(e))?;

        let filename = match path.file_name() {
            Some(name) => name,
            None => return Err(FileError::InvalidFilepath)
        }

        let mut file = Self { filename, fileID: FileID::new(filename), }

        Ok(file)
    }
}

impl Hashable for File {
    fn hash(&self) -> Hash {}
}
