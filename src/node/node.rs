use libp2p::{
    identity,
    kad::{record::store::MemoryStore, Kademlia, KademliaEvent, QueryResult},
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour, PeerId, Swarm,
};

use async_std::task;
use futures::prelude::*;
use std::{
    error::Error,
    fs,
    io::Write,
    path::Path,
    str::from_utf8,
    task::{Context, Poll},
};

use super::super::{common, common::Stack, primitives::file, GeneralError};

/// The main network behavior for the Meros protocol.
#[derive(NetworkBehaviour)]
struct MerosBehavior {
    kademlia: Kademlia<MemoryStore>,
    mdns: Mdns,
}

impl NetworkBehaviourEventProcess<MdnsEvent> for MerosBehavior {
    fn inject_event(&mut self, event: MdnsEvent) {
        // Add the discovered peers to the dht
        if let MdnsEvent::Discovered(discovered_peers) = event {
            for (peer_id, multiaddr) in discovered_peers {
                self.kademlia.add_address(&peer_id, multiaddr);
            }
        }
    }
}

impl NetworkBehaviourEventProcess<KademliaEvent> for MerosBehavior {
    fn inject_event(&mut self, event: KademliaEvent) {
        match event {
            // If the event is a query
            KademliaEvent::QueryResult { id, result, stats } => {
                match result {
                    // If the query is a GET
                    QueryResult::GetRecord(Ok(ok)) => {
                        for query in ok.records {
                            println!(
                                "got record {:?} {:?}",
                                from_utf8(query.record.key.as_ref()).unwrap(),
                                from_utf8(&query.record.value).unwrap()
                            );
                        }
                    }

                    // If the query is a failed GET
                    QueryResult::GetRecord(Err(err)) => {
                        eprintln!("failed to get record: {:?}", err);
                    }

                    // If the query is a PUT
                    QueryResult::PutRecord(Ok(ok)) => {
                        println!(
                            "put record {:?}",
                            from_utf8(ok.key.as_ref()).unwrap()
                        );
                    }

                    // If the query is a failed PUT
                    QueryResult::PutRecord(Err(err)) => {
                        eprintln!("failed to put record: {:?}", err);
                    }

                    _ => {}
                }
            }
            _ => {}
        }
    }
}

impl crate::CanSerialize for PeerId {
    type S = Self;
    fn to_bytes(&self) -> bincode::Result<Vec<u8>> {
        Ok(self.into_bytes())
    }
    fn from_bytes(bytes: Vec<u8>) -> bincode::Result<Self> {
        Ok(PeerId::from_bytes(bytes).unwrap())
    }
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

/// Nodes can have different functions. This enum differentiates between
/// different kinds of node behaviors.
enum NodeType {
    /// Is not a real node, only sends requests to the network. Does not
    /// host any shards.
    Client {
        /// The node's list of pending operations.
        pending_operations: Stack<Operation>,
    },

    /// A node that only exists to broadcast and store shrads. It cannot
    /// make requests on the network.
    Node { trust: u32 },
}

impl Default for NodeType {
    fn default() -> Self {
        let pending_operations: Stack<Operation> = Stack::new();
        NodeType::Client { pending_operations }
    }
}

/// A node on the Meros network. A Node can make requests to the network to
/// get, put, update, and delete files.
pub struct Node {
    /// The node's libp2p ed25519 keypair
    keypair: identity::Keypair,

    /// The node's libp2p PeerId (hash of the pubk)
    peer_id: PeerId,

    /// The type of node that this node is, and other node type-specific
    /// information.
    node_type: NodeType,
}

impl Node {
    /// Initialize a new node.
    /// # Arguments
    /// * `name` - The local name of the node on the disk.
    pub fn new(name: &str) -> Result<Self, Box<dyn Error>> {
        let path = Path::new(common::DATADIR).join("identities").join(name);

        // If the identity already exists, load it from disk
        if path.exists() {
            let keypair =
                identity::Keypair::Ed25519(identity::ed25519::Keypair::decode(
                    &mut fs::read(path.join("keypair"))?,
                )?);
            return Ok(Node {
                peer_id: PeerId::from_public_key(keypair.public()),
                keypair,
                node_type: NodeType::default(),
            });
        }

        // If it does not, create it and persist it to disk
        let keypair = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from_public_key(keypair.public());
        fs::create_dir_all(&path)?;
        if let identity::Keypair::Ed25519(k) = &keypair {
            fs::File::create(&path.join("keypair"))?.write_all(&k.encode())?;
            return Ok(Node {
                peer_id,
                keypair,
                node_type: NodeType::default(),
            });
        }
        Err(Box::new(GeneralError {
            details: String::from("error creating new node"),
        }))
    }

    /// Start listening on a node
    pub fn start_listening(&mut self, port: u16) -> Result<(), Box<dyn Error>> {
        // Build the swarm
        let transport =
            libp2p::build_tcp_ws_secio_mplex_yamux(self.keypair.clone())?;

        let mut swarm = {
            let kademlia = {
                let store = MemoryStore::new(self.peer_id.clone());
                Kademlia::new(self.peer_id.clone(), store)
            };

            let mdns = Mdns::new()?;
            let behavior = MerosBehavior { kademlia, mdns };

            Swarm::new(transport, behavior, self.peer_id.clone())
        };

        // Start listening on this node
        Swarm::listen_on(
            &mut swarm,
            format!("/ip4/0.0.0.0/tcp/{}", port).parse()?,
        )?;

        // Construct the future for handling lines from stdin
        let mut listening = false;
        let fut = future::poll_fn(move |cx: &mut Context<'_>| {
            loop {
                match self.node_type {
                    // Execute all the pending operations
                    NodeType::Client {
                        pending_operations, ..
                    } => {
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
                    }

                    // Do node stuff (what does the node need to do?)
                    NodeType::Node { .. } => {}
                }

                // Poll the swarm for an event
                match swarm.poll_next_unpin(cx) {
                    Poll::Ready(Some(event)) => {
                        println!("swarm event: {:?}", event)
                    }
                    Poll::Ready(None) => return Poll::Ready(Ok(())),
                    Poll::Pending => {
                        if !listening {
                            if let Some(addr) = Swarm::listeners(&swarm).next() {
                                println!("listening on {:?}", addr);
                                listening = true;
                            }
                        }
                        break;
                    }
                }
            }
            Poll::Pending
        });

        task::block_on(fut)
    }

    fn get_file(&self, file_id: file::FileID, config: OperationConfig);
    fn put_file(
        &self,
        file: file::File,
        file_bytes: Vec<u8>,
        config: OperationConfig,
    );
}
