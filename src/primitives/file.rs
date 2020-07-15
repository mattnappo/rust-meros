use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

use crate::crypto::hash;
use crate::db::{IsKey, IsValue};

/// The structure used for the identification of a file on the meros
/// network.
#[derive(Debug, Serialize, Deserialize)]
pub struct FileID(hash::Hash);

impl FileID {
    fn new(
        filename: &str,
        bytes: &Vec<u8>,
    ) -> Result<Self, SystemTimeError> {
        let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()
            as u128;

        let data =
            [filename.as_bytes(), &bytes[..], time.to_string().as_bytes()]
                .concat()
                .to_vec();
        Ok(Self(hash::hash_bytes(data)))
    }
}

impl IsKey for FileID {}

/// All possible errors that could be returned from `File`'s methods.
#[derive(Debug)]
enum FileError {
    IO(std::io::Error),
    InvalidFilepath(crate::GeneralError),
    SystemTimeError(SystemTimeError),
}

/// The structure representing a file on the meros network. This structure
/// contains valuable information about a file, but does not contain the data
/// of the file. Rather, that is stored amongst the nodes described in the
/// `shard_db` field.
#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub filename: String,
    // shard_db: Option<database::Database<Shard>>,
    pub id: FileID,
}

impl File {
    /// Abstraction method to generate the metadata of a `File`.
    /// This method does not distribute a file over the meros network.
    /// However, it does prepare the file for sharding by pre-calculating
    /// the shards and assigning them to null nodes (temporarily).
    fn new(path: &std::path::Path) -> Result<Self, FileError> {
        let mut file =
            std::fs::File::open(path).map_err(|e| FileError::IO(e))?;

        let mut buf = Vec::new();
        file.read_to_end(&mut buf).map_err(|e| FileError::IO(e))?;

        let invalid_path =
            Err(FileError::InvalidFilepath(crate::GeneralError::new(
                format!("{} is an invalid filepath", path.display())
                    .as_str(),
            )));

        // clean this up somehow
        let filename = match path.file_name() {
            Some(name) => match name.to_str() {
                Some(s) => s,
                None => return invalid_path,
            },
            None => return invalid_path,
        };

        let file = Self {
            filename: filename.to_string(),
            id: FileID::new(filename, &buf)
                .map_err(|e| FileError::SystemTimeError(e))?,
        };

        Ok(file)
    }
}

impl super::Hashable for File {
    fn hash(&self) -> hash::Hash {
        [0 as u8; 32] // temp
    }
}

impl IsValue for File {}

impl crate::crypto::encryption::CanEncrypt for File {}

impl crate::CanSerialize for File {
    type S = Self;

    fn to_bytes(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)
    }

    fn from_bytes(bytes: Vec<u8>) -> bincode::Result<Self> {
        bincode::deserialize(&bytes[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_new_file() {
        File::new(Path::new("testfile.txt")).unwrap();
    }
}
