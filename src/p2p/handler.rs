use libp2p::{
    floodsub,
    kad::{
        record::{store::MemoryStore, Key},
        Kademlia, Quorum, Record,
    },
};

use crate::primitives::file;
use crate::CanSerialize;

// This is what drives the client. When a client wants to publish a file, it
// will do this (mainly the kademlia.get_record and kademlia.put_record)
pub fn handle_stdin_line(
    kademlia: &mut Kademlia<MemoryStore>,
    floodsub: &mut floodsub::Floodsub,
    line: String,
) {
    let mut args = line.split(" ");
    match args.next() {
        Some("GET") => {
            let key = match args.next() {
                Some(key) => {
                    let raw = file::FileID::from_hex(key)
                        .expect("could not deserialzie fileid");
                    Key::new(&raw.to_bytes().unwrap())
                }
                None => {
                    eprintln!("expected a key");
                    return;
                }
            };

            kademlia.get_record(&key, Quorum::One);
        }
        Some("PUT") => {
            let key = match args.next() {
                Some(key) => Key::new(&key),
                None => {
                    eprintln!("Expected a key");
                    return;
                }
            };

            let value = match args.next() {
                Some(value) => value.as_bytes().to_vec(),
                None => {
                    eprintln!("Expected value");
                    return;
                }
            };

            let record = Record {
                key,
                value,
                publisher: None,
                expires: None,
            };

            kademlia
                .put_record(record, Quorum::One)
                .expect("Failed to store record locally");

            floodsub.publish(
                floodsub::Topic::new(super::node::SHARD_CHANNEL),
                "published a record via stdin".as_bytes(),
            );
        }
        _ => {
            eprintln!("Expected GET or PUT");
        }
    }
}
