use serde::{Deserialize, Serialize};
use std::{
    cmp::PartialEq,
    io::prelude::*,
    time::{SystemTime, SystemTimeError, UNIX_EPOCH},
};

use crate::{
    crypto::{encryption::CanEncrypt, hash},
    db::{IsKey, IsValue},
};

/// The structure used for the identification of a file on the meros
/// network.
#[derive(Debug, Serialize, Deserialize)]
pub struct FileID {
    id: hash::Hash,
}

impl FileID {
    pub fn new(
        filename: &str,
        bytes: &Vec<u8>,
    ) -> Result<Self, SystemTimeError> {
        let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()
            as u128;

        let data =
            [filename.as_bytes(), &bytes[..], time.to_string().as_bytes()]
                .concat()
                .to_vec();
        Ok(Self {
            id: hash::hash_bytes(data),
        })
    }
}

impl PartialEq for FileID {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl IsKey for FileID {}

/// All possible errors that could be returned from `File`'s methods.
#[derive(Debug)]
pub enum FileError {
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
    pub fn new(path: &std::path::Path) -> Result<Self, FileError> {
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

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.filename == other.filename && self.id == other.id
    }
}
impl super::Hashable for File {
    fn hash(&self) -> hash::Hash {
        [0 as u8; 32] // temp
    }
}

impl IsValue for File {}

impl crate::CanSerialize for File {
    type S = Self;
    fn to_bytes(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)
    }
    fn from_bytes(bytes: Vec<u8>) -> bincode::Result<Self::S> {
        bincode::deserialize(&bytes[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CanSerialize;
    use std::path::Path;

    #[test]
    fn test_new_file() {
        File::new(Path::new("testfile.txt")).unwrap();
    }

    #[test]
    fn test_to_bytes() {
        let file = File::new(Path::new("testfile.txt")).unwrap();
        let bytes = file.to_bytes().unwrap();
        println!("bytes: {:?}", bytes);
    }

    #[test]
    fn test_from_bytes() {
        let file = File::new(Path::new("testfile.txt")).unwrap();
        let serialized = file.to_bytes().unwrap();

        let deserialized = File::from_bytes(serialized).unwrap();
        println!("deserialized file: {:?}", deserialized);

        // assert_eq!(file, deserialized);
    }
}
