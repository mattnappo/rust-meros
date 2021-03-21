use libp2p::{
    kad::{record::store::MemoryStore, Kademlia, KademliaEvent, QueryResult},
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour, PeerId, Swarm,
};

use async_std::task;
use futures::prelude::*;
use std::{
    error::Error,
    str::from_utf8,
    task::{Context, Poll},
};

use super::identity::Identity;

/// The main network behavior for the Meros protocol.
#[derive(NetworkBehaviour)]
struct MerosBehavior {
    /// The main Kademlia DHT, which stores metadata to files and shards
    kademlia: Kademlia<MemoryStore>,

    /// Mdns instance for peer discovery
    mdns: Mdns,
}

impl NetworkBehaviourEventProcess<MdnsEvent> for MerosBehavior {
    /// Upon an Mdns event
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
    /// Upon a Kademlia event
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

/// A node on the Meros network. A Node stores and broadcasts shards on the network
/// to host files.
pub struct Node {
    /// The node's identity and private key on the network (keypair and peer id)
    identity: Identity,
    // The collection of shards that this node holds.
    // shards: ShardStore Map between fileID and a Vec of shards, but a sled db. Need to
    // re-write the sled db code (its garbage).
}

impl Node {
    /// Initialize a new node.
    /// # Arguments
    /// * `name` - The local name of the node on the disk.
    pub fn new(name: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Node {
            identity: Identity::new(name)?,
        })
    }

    /// Start listening on a node
    pub fn start_listening(&mut self, port: u16) -> Result<(), Box<dyn Error>> {
        // Build the swarm
        let transport =
            libp2p::build_tcp_ws_secio_mplex_yamux(self.identity.keypair.clone())?;

        let mut swarm = {
            let kademlia = {
                let store = MemoryStore::new(self.identity.peer_id.clone());
                Kademlia::new(self.identity.peer_id.clone(), store)
            };

            let mdns = Mdns::new()?;
            let behavior = MerosBehavior { kademlia, mdns };

            Swarm::new(transport, behavior, self.identity.peer_id.clone())
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
                // First do node stuff (what does the node need to do?)

                // Then poll the swarm for an event
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
}
