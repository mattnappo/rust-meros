use rust_meros::p2p::node::{Node, Operation, OperationConfig};
use rust_meros::{
    crypto::encryption,
    primitives::{file, shard},
};
use std::path::Path;
use std::error::Error;
use async_std;

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

/// Run a test node with a test put operation (this will be replaced with a better
/// interface, CLI or CL flags)
async fn run_node() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 3 {
        let mut node = Node::new(args[1].as_str()).unwrap();

        if args.len() == 4 && args[3] == "test-put" {
            node.push_operation(get_test_operation());
        }

        return node.start_listening(args[2].parse::<u16>().unwrap())
            .await
    }
    panic!("must specify an identity and a port");
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run_node().await
}
