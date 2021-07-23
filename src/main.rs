use rust_meros::p2p::node::{Node, Operation, OperationConfig};
use rust_meros::{
    crypto::encryption,
    primitives::{file, shard},
};
use std::path::Path;

/// Create a test operation
fn get_test_operation() -> Operation {
    let (sk1, pk1) = encryption::gen_keypair("testkey", false).unwrap();
    let (file, shards) = file::File::new(
        Path::new("testfile.txt"),
        shard::ShardConfig::new(5, &pk1),
        &sk1,
    )
    .unwrap();

    let bytes =
        shard::Shard::reconstruct(&shards, &shard::ShardConfig::new(5, &pk1), None)
            .unwrap();

    Operation::PutFile {
        file_metadata: file,
        file_bytes: bytes,
        config: OperationConfig {
            output_file: "none".to_string(),
            min_nodes: 0,
            decompress: false,
            decrypt: false,
        },
    }
}

fn run_node() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 {
        let mut node = Node::new(args[1].as_str()).unwrap();

        node.push_operation(get_test_operation());

        node.start_listening(args[2].parse::<u16>().unwrap())
            .unwrap();
    }
    panic!("must specify an identity and a port");
}

fn main() {
    run_node();
}
