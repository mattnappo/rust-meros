use crate::CanSerialize;
use libp2p::{
    kad::{
        mut record::{store::MemoryStore, Key},
        Kademlia, KademliaEvent, QueryResult, Quorum, Record,
    },
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    swarm::SwarmEvent,
    NetworkBehaviour, PeerId, Swarm, Multiaddr,
};

use async_std::task;
use futures::prelude::*;
use std::{
    error::Error,
    str::from_utf8,
    task::{Context, Poll},
};

use super::identity::Identity;
use super::store::ShardStore;
use crate::{common::Stack, primitives::{file, shard}};

/// The main network behavior for the Meros protocol.
#[derive(NetworkBehaviour)]
struct MerosBehavior {
    /// The main Kademlia DHT, which stores metadata to files and shards
    kademlia: Kademlia<MemoryStore>,

    /// Mdns instance for peer discovery
    mdns: Mdns,
}

impl MerosBehavior {
    /// Get a list of the alive peers in the DHT.
    pub fn get_online_peers(&mut self) -> Vec<PeerId> {
        let mut nodes: Vec<PeerId> = Vec::new();
        let buckets = self.kademlia.kbuckets();
        for bucket in buckets {
            for node in bucket.iter() {
                let id = node.to_owned().node.key.into_preimage();
                if !nodes.contains(&id) {
                    nodes.push(id);
                }
            }

        }
        nodes
    }
}


impl NetworkBehaviourEventProcess<MdnsEvent> for MerosBehavior {
    /// Upon an Mdns event
    fn inject_event(&mut self, event: MdnsEvent) {
        // Add the discovered peers to the dht and remove stale peers
        match event {
            MdnsEvent::Discovered(discovered_peers) => {
                for (peer_id, multiaddr) in discovered_peers {
                    self.kademlia.add_address(&peer_id, multiaddr);
                    println!("found peer {:?}", peer_id);
                }
            }
            MdnsEvent::Expired(expired_peers) => {
                for (peer_id, _) in expired_peers {
                    self.kademlia.remove_peer(&peer_id);
                    println!("removed peer {:?}", peer_id);
                }
            }
        }
    }
}

impl NetworkBehaviourEventProcess<KademliaEvent> for MerosBehavior {
    /// Upon a Kademlia event
    fn inject_event(&mut self, event: KademliaEvent) {
        match event {
            // If the event is a query
            KademliaEvent::OutboundQueryCompleted { result, .. } => {
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

/// A node on the Meros network. A Node stores and broadcasts shards on the network
/// to host files.
pub struct Node {
    /// The node's identity and private key on the network (keypair and peer id)
    identity: Identity,

    /// The collection of shards that this node holds. This is a map from
    /// fileIDs to a Vec of shards, using sled db.
    shards: ShardStore, // Make Arc<RwLock<>>

    /// This node's list of pending operations.
    pending_ops: Stack<Operation>, // Make Arc<RwLock<>>

    visible_peers: Vec<PeerId>,
}

/// An operation that a node on the network can perform. This enum will
/// grow as features on the network grow.
pub enum Operation {
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
pub struct OperationConfig {
    /// Output location for a get file request on the disk
    pub output_file: String,

    /// Minimum number of nodes that the operation must contact to be valid.
    pub min_nodes: u16,

    /// Should the output be automatically decompressed.
    pub decompress: bool,

    /// Should the output be automatically decrypted.
    pub decrypt: bool,
}

impl Node {
    /// Initialize a new node.
    /// # Arguments
    /// * `name` - The local name of the node on the disk.
    pub fn new(name: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Node {
            identity: Identity::new(name)?,
            shards: ShardStore::new(name)?,
            pending_ops: Stack::new(),
            visible_peers: Vec::new(),
        })
    }

    /// Push a network operation to this node's stack of operations.
    pub fn push_operation(&mut self, op: Operation) {
        self.pending_ops.push(op);
    }

    /// Start listening on a node
    pub async fn start_listening(&mut self, port: u16) -> Result<(), Box<dyn Error>> {
        // Build the swarm
        let transport = libp2p::development_transport(self.identity.keypair.clone()).await?;

        let mut swarm = {
            let kademlia = {
                let store = MemoryStore::new(self.identity.peer_id.clone());
                Kademlia::new(self.identity.peer_id.clone(), store)
            };
            let mdns = task::block_on(Mdns::new(MdnsConfig::default()))?;
            let behavior = MerosBehavior { kademlia, mdns };

            Swarm::new(transport, behavior, self.identity.peer_id.clone())
        };

        // Start listening on this node
        Swarm::listen_on(&mut swarm, format!("/ip4/0.0.0.0/tcp/{}", port).parse()?)?;

        // Construct the future for handling lines from stdin
        let mut listening = false;
        let fut = future::poll_fn(move |cx: &mut Context<'_>| {
            loop {
                // If this node has pending operations, execute them
                match self.pending_ops.pop() {
                    Some(op) => {
                        match op {
                            Operation::PutFile {
                                file_metadata,
                                file_bytes,
                                config,
                            } => self.put_file(
                                //&mut swarm.kademlia,
                                &mut swarm,
                                file_metadata,
                                file_bytes.to_vec(),
                                &config,
                            ),
                            Operation::GetFile { file_id, config } => self.get_file(
                                &mut swarm,
                                &file_id,
                                &config,
                            ),
                        }
                    }
                    None => {}
                }

                // Then poll the swarm for an event
                match swarm.poll_next_unpin(cx) {
                    Poll::Ready(Some(event)) => {
                        match event {
                            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                                println!("peer joined: {:?}", peer_id);
                            }

                            SwarmEvent::ConnectionClosed { peer_id, .. } => println!("peer left: {:?}", peer_id),

                            _ => println!("swarm event: {:?}", event),
                        }
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

    /// Core node operation to put a file onto the network.
    fn put_file(
        &mut self,
        //kad: &mut Kademlia<MemoryStore>,
        swarm: &mut Swarm<MerosBehavior>,
        file_metadata: file::File,
        file_bytes: Vec<u8>,
        config: &OperationConfig,
    ) -> Result<(), Box<dyn Error>> {
        /*
           1. Find online peers, get their peerIDs, and modify the metadata to
           include the shard locations

           2. Put the metadata into the DHT

           3. Contact the peers with the shard data (the shards of the actual bytes
              of the file).
        */
        println!("putting file");

        // (1) Get the online peers
        let mut peers = swarm.behaviour_mut().get_online_peers();
        if peers.len() > super::MAX_SHARDS {
            peers.truncate(super::MAX_SHARDS);
        }

        // Calcualte the shards of the file and update file sharding metadata accordingly
        file_metadata.set_shards(peers);
        let (shards, new_config) =
            shard::Shard::shard(&file_bytes, file_metadata.shard_config)?;

        file_metadata.shard_config = new_config;

        // (2) Insert into the DHT the FileID which points to the relevant metadata.
        let record = Record {
            key: Key::new(&file_metadata.id.to_bytes().unwrap()),
            value: file_metadata.to_bytes().unwrap(),
            publisher: Some(self.identity.peer_id.clone()),
            expires: None,
        };
        swarm
            .behaviour_mut()
            .kademlia
            .put_record(record, Quorum::One)
            .expect("Failed to store the record");

        // (3) Then distribute the actual file bytes data across the network.

        Ok(())
    }

    /// Core node operation to get a file from the network.
    fn get_file(
        &mut self,
        swarm: &mut Swarm<MerosBehavior>,
        file_id: &file::FileID,
        config: &OperationConfig,
    ) {
    }
}
