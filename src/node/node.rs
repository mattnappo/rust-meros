use libp2p::{
    build_development_transport, identity,
    kad::{
        record::store::MemoryStore, Kademlia, KademliaEvent, PeerRecord,
        PutRecordOk, QueryResult, Record,
    },
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour, PeerId, Swarm,
};

use async_std::{io, task};
use futures::prelude::*;
use std::{
    error::Error,
    fs,
    io::{Read, Write},
    path::Path,
    str::from_utf8,
    task::{Context, Poll},
};

use super::super::common;
use super::super::crypto;
use super::super::GeneralError;
use super::handler;

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

/// A node on the Meros network. A Node can make requests to the network to
/// get, put, update, and delete files.
pub struct Node {
    keypair: identity::Keypair,
    peer_id: PeerId,
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
                keypair: keypair,
            });
        }

        // If it does not, create it and persist it to disk
        let keypair = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from_public_key(keypair.public());
        fs::create_dir_all(&path)?;
        if let identity::Keypair::Ed25519(k) = &keypair {
            fs::File::create(&path.join("keypair"))?.write_all(&k.encode())?;
            return Ok(Node {
                peer_id: peer_id,
                keypair,
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
        let mut stdin = io::BufReader::new(io::stdin()).lines();
        task::block_on(future::poll_fn(move |cx: &mut Context<'_>| {
            // Poll stdin
            loop {
                match stdin.try_poll_next_unpin(cx)? {
                    Poll::Ready(Some(line)) => {
                        handler::handle_stdin_line(&mut swarm.kademlia, line)
                    }

                    Poll::Ready(None) => panic!("stdin closed (errored)"),
                    Poll::Pending => break,
                }
            }
            loop {
                /*
                    if self.pending_ops.len() > 0
                    then get/push each file.
                */

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
        }))
    }
}
