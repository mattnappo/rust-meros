use std::io::prelude::*;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

use crate::crypto;

/// The structure used for the identification of a file on the meros
/// network. It is a hash calculated in a specific way, as described in
/// the `File` implementation.
pub struct FileID(crypto::Hash);

impl FileID {
    fn new(filename: &str) -> Result<Self, SystemTimeError> {
        let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()
            as u128;

        let data = [filename.as_bytes(), time.to_string().as_bytes()]
            .concat()
            .to_vec();
        Ok(Self(crypto::hash_bytes(data)))
    }
}

/// All possible errors that could be returned from `File`'s methods.
enum FileError {
    IO(std::io::Error),
    InvalidFilepath(super::GenericError),
    SystemTimeError(SystemTimeError),
}

/// data of the file is stored at the nodes described in the `File`'s
/// `shard_db` field.
pub struct File {
    pub filename: String,
    pub file_id: FileID,
    // shard_db: Option<database::Database<Shard>>,
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

        let filename = match path.file_name() {
            Some(name) => match name.to_str() {
                Some(s) => s,
                None => {
                    return Err(FileError::InvalidFilepath(
                        super::GenericError::new(
                            format!(
                                "{} is an invalid filepath",
                                path.display()
                            )
                            .as_str(),
                        ),
                    ))
                }
            },
            None => {
                return Err(FileError::InvalidFilepath(
                    super::GenericError::new(
                        format!(
                            "{} is an invalid filepath",
                            path.display()
                        )
                        .as_str(),
                    ),
                ))
            }
        };

        let file = Self {
            filename: filename.to_string(),
            file_id: FileID::new(filename)
                .map_err(|e| FileError::SystemTimeError(e))?,
        };

        Ok(file)
    }
}

impl super::Hashable for File {
    fn hash(&self) -> crypto::Hash {
        [0 as u8; 32] // temp
    }
}
