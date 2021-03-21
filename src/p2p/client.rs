use super::identity;
use crate::{common::Stack, primitives::file};
use std::error::Error;

/// A client on the network does not host any shards. Rather, it just makes
/// requests to the network to get or put files.
struct Client {
    /// This client's list of pending operations.
    pending_ops: Stack<Operation>, // Make Arc<RwLock<>>

    /// The client's identity
    identity: identity::Identity,
}

/// An operation that a node/client on the network can perform. This enum will
/// grow as features on the network grow.
#[derive(Debug)]
enum Operation {
    /// Store a file on the network. Also sends the shards to all other nodes.
    PutFile {
        file_metadata: file::File,
        file_bytes: Vec<u8>,
        config: OperationConfig,
    },

    /// Poll all the necessary nodes to get a file from the network.
    GetFile {
        file_id: file::FileID,
        config: OperationConfig,
    },
}

/// Parameters for a client operation on the network.
#[derive(Debug)]
struct OperationConfig {
    /// Output location for a get file request on the disk
    pub output_file: String,

    /// Minimum number of nodes that the operation must contact to be valid.
    pub min_nodes: u16,

    /// Should the output be automatically decompressed.
    pub decompress: bool,

    /// Should the output be automatically decrypted.
    pub decrypt: bool,
}

impl Client {
    fn new(name: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Client {
            pending_ops: Stack::new(),
            identity: identity::Identity::new(name)?,
        })
    }

    fn get_file(&self, file_id: file::FileID, config: OperationConfig);

    fn put_file(
        &self,
        file: file::File,
        file_bytes: Vec<u8>,
        config: OperationConfig,
    );
}
/*

                        for op in pending_operations.vec().into_iter() {
                            match op {
                                Operation::PutFile {
                                    file_metadata,
                                    file_bytes,
                                    config,
                                } => self.put_file(
                                    file_metadata,
                                    file_bytes,
                                    config,
                                ),
                                Operation::GetFile { file_id, config } => {
                                    self.get_file(file_id, config)
                                }
                            }
                        }
*/
