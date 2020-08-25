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
    str::from_utf8,
    task::{Context, Poll},
};

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
                                from_utf8(query.record.key.as_ref())
                                    .unwrap(),
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
/// Initialize a node.
/// TODO: This should take a public key and a port to listen on as a parameter.
pub fn init_node() -> Box<dyn Error> {
    // Generate an identity
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    let transport = build_development_transport(local_key)?;

    let mut swarm = {
        let kademlia = {
            let store = MemoryStore::new(local_peer_id.clone());
            Kademlia::new(local_peer_id.clone(), store)
        };

        let mdns = Mdns::new()?;
        let mehavior = MerosBehavior { kademlia, mdns };

        Swarm::new(transport, behavior, local_peer_id)
    };

    // Start listening on this node
    Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Construct the future for handling lines from stdin
    let mut printed_listen = false;
    let mut stdin = io::BufReader::new(io::stdin()).lines();
    let handler_future = future::poll_fn(move |cx: &mut Context<'_>| {
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

        // Poll the swarm for an event
        match swarm.poll_next_unpin(cx) {
            Poll::Ready(Some(event)) => {
                println!("swarm event: {:?}", event)
            }

            Poll::Ready(None) => {
                return Poll::Ready(Ok(()));
            }

            Poll::Pending => {
                if !printed_listen {
                    if let Some(addr) = Swarm::listeners(&swarm).next() {
                        println!("listening on {:?}", addr);
                        printed_listen = true;
                    }
                }
            }
        }

        Poll::Pending
    });

    task::block_on(handler_future)
}
