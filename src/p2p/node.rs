use crate::CanSerialize;
use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent},
    kad::{
        record::{store::MemoryStore, Key},
        Kademlia, KademliaEvent, QueryResult, Quorum, Record,
    },
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::{NetworkBehaviourEventProcess, SwarmEvent},
    NetworkBehaviour, PeerId, Swarm,
};

use async_std::task;
use futures::prelude::*;
use std::{
    clone::Clone,
    error::Error,
    task::{Context, Poll},
};

use super::identity::Identity;
use super::store::ShardStore;
use crate::{
    primitives::{file, shard},
    GeneralError,
};

/// The floodsub topic string where shards are exchanged.
const SHARD_CHANNEL: &str = "shard_channel";

/// The main network behavior for the Meros protocol.
#[derive(NetworkBehaviour)]
struct MerosBehavior {
    /// The main Kademlia DHT, which stores metadata to files and shards
    kademlia: Kademlia<MemoryStore>,

    /// Mdns instance for peer discovery
    mdns: Mdns,

    /// Floodsub for communicating shard data
    floodsub: Floodsub,
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
                    self.floodsub.add_node_to_partial_view(peer_id);
                    println!("found peer {:?}", peer_id);
                }
            }
            MdnsEvent::Expired(expired_peers) => {
                for (peer_id, _) in expired_peers {
                    self.kademlia.remove_peer(&peer_id);
                    if !self.mdns.has_node(&peer_id) {
                        self.floodsub.remove_node_from_partial_view(&peer_id);
                    }
                    println!("removed peer {:?}", peer_id);
                }
            }
        }
    }
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for MerosBehavior {
    /// Upon a floodsub event
    fn inject_event(&mut self, event: FloodsubEvent) {
        match event {
            FloodsubEvent::Message(msg) => {
                println!(
                    "\n=========\nreceived msg: {:?} from {:?}\n========\n",
                    String::from_utf8_lossy(&msg.data),
                    msg.source
                );
            }
            _ => println!("FLOODSUB EVENT: {:?}", event),
        };
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
                                "KAD EVENT: got record {:?} {:?}",
                                query.record.key.as_ref(),
                                &query.record.value
                            );
                        }
                    }
                    // If the query is a failed GET
                    QueryResult::GetRecord(Err(err)) => {
                        eprintln!("failed to get record: {:?}", err);
                    }

                    // If the query is a PUT
                    QueryResult::PutRecord(Ok(ok)) => {
                        println!("KAD EVENT: put record {:?}", ok.key.as_ref());
                    }

                    // If the query is a failed PUT
                    QueryResult::PutRecord(Err(err)) => {
                        eprintln!("KAD EVENT: failed to put record: {:?}", err);
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
    pending_ops: Vec<Operation>, // Make Arc<RwLock<>>
}

/// An operation that a node on the network can perform. This enum will
/// grow as features on the network grow.
#[derive(Clone)]
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

    /// Send a test floodsub msg.
    TestSub,
}

/// Parameters for a client operation on the network.
#[derive(Debug, Clone)]
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
            pending_ops: Vec::new(),
        })
    }

    /// Push a network operation to this node's stack of operations.
    pub fn push_operation(&mut self, op: Operation) {
        self.pending_ops.push(op);
    }

    /// Start listening on a node
    pub async fn start_listening(
        &mut self,
        port: u16,
    ) -> Result<(), Box<dyn Error>> {
        // Build the swarm
        let transport =
            libp2p::development_transport(self.identity.keypair.clone()).await?;

        let shard_channel = floodsub::Topic::new(SHARD_CHANNEL);

        let mut swarm = {
            let kademlia = {
                let store = MemoryStore::new(self.identity.peer_id.clone());
                Kademlia::new(self.identity.peer_id.clone(), store)
            };
            let mdns = task::block_on(Mdns::new(MdnsConfig::default()))?;
            let floodsub = Floodsub::new(self.identity.peer_id.clone());
            let mut behavior = MerosBehavior {
                kademlia,
                mdns,
                floodsub,
            };

            if behavior.floodsub.subscribe(shard_channel.clone()) == true {
                println!("SUBSCRIBED SUCCESSFULLY");
            } else {
                println!("did not subscribe");
            }

            Swarm::new(transport, behavior, self.identity.peer_id.clone())
        };

        // Start listening on this node
        Swarm::listen_on(&mut swarm, format!("/ip4/0.0.0.0/tcp/{}", port).parse()?)?;

        // Construct the future for handling lines from stdin
        let mut listening = false;
        let fut = future::poll_fn(move |cx: &mut Context<'_>| {
            loop {
                // If this node has pending operations, execute them
                if self.pending_ops.len() != 0 {
                    let result = match self.pending_ops[0].clone() {
                        Operation::PutFile {
                            file_metadata,
                            file_bytes,
                            config,
                        } => self.put_file(
                            &mut swarm,
                            file_metadata,
                            file_bytes.to_vec(),
                            &config,
                        ),
                        Operation::TestSub => self.test_sub(&mut swarm),
                        _ => Ok(()),
                    };

                    match result {
                        Ok(_) => {
                            println!("successfully executed operation");
                            self.pending_ops.remove(0);
                        }
                        Err(e) => println!("error executing operation: {:?}", e),
                    }
                }

                // Then poll the swarm for an event
                match swarm.poll_next_unpin(cx) {
                    Poll::Ready(Some(event)) => match event {
                        SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                            println!("peer joined: {:?}", peer_id);
                        }

                        SwarmEvent::ConnectionClosed { peer_id, .. } => {
                            println!("peer left: {:?}", peer_id)
                        }

                        _ => println!("swarm event: {:?}", event),
                    },
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
        swarm: &mut Swarm<MerosBehavior>,
        mut file_metadata: file::File,
        file_bytes: Vec<u8>,
        _: &OperationConfig,
    ) -> Result<(), Box<dyn Error>> {
        /*
           1. Find online peers, get their peerIDs, and modify the metadata to
           include the shard locations

           2. Put the metadata into the DHT

           3. Contact the peers with the shard data (the shards of the actual bytes
              of the file).
        */

        // (1) Get the online peers
        let mut peers = swarm.behaviour_mut().get_online_peers();
        if peers.len() > super::MAX_SHARDS {
            peers.truncate(super::MAX_SHARDS);
        }

        if peers.len() == 0 {
            return Err(Box::new(GeneralError::new(
                "not enough peers to shard file",
            )));
        }

        // Calcualte the shards of the file and update file sharding metadata accordingly
        file_metadata.set_shards(&peers);
        let (shards, new_config) =
            shard::Shard::shard(&file_bytes, file_metadata.shard_config)?;

        file_metadata.shard_config = new_config;

        // (2) Insert into the DHT the FileID which points to the relevant metadata.
        let record = Record {
            key: Key::new(&file_metadata.id.to_bytes()?),
            value: file_metadata.to_bytes()?,
            publisher: Some(self.identity.peer_id.clone()),
            expires: None,
        };
        swarm
            .behaviour_mut()
            .kademlia
            .put_record(record, Quorum::One)?;

        // (3) Then distribute the actual file bytes data across the network.
        //for peer in &peers {
        //    swarm.dial(peer)?;
        //}
        //swarm.behaviour_mut().floodsub.publish_any(
        //    floodsub::Topic::new(SHARD_CHANNEL),
        //    "test message".as_bytes(),
        //);

        Ok(())
    }

    fn test_sub(
        &mut self,
        swarm: &mut Swarm<MerosBehavior>,
    ) -> Result<(), Box<dyn Error>> {
        println!("testing sub");
        swarm.behaviour_mut().floodsub.publish(
            floodsub::Topic::new(SHARD_CHANNEL),
            "test message".as_bytes(),
        );
        Ok(())
    }

    /// Core node operation to get a file from the network.
    fn get_file(
        &mut self,
        swarm: &mut Swarm<MerosBehavior>,
        file_id: &file::FileID,
        config: &OperationConfig,
    ) -> Result<(), Box<dyn Error>> {
        println!("getting file");

        let qid = swarm
            .behaviour_mut()
            .kademlia
            .get_record(&Key::new(&file_id.to_bytes()?), Quorum::One);

        let query = swarm.behaviour_mut().kademlia.query(&qid);
        match query {
            Some(q) => println!(
                "--------------QUERY:\n{:#?}\n\n{:#?}\n------------",
                q.info(),
                q.stats()
            ),
            None => println!("QUERY FAILED"),
        }

        Ok(())
    }
}
