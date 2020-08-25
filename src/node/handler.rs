use libp2p::{
    identity,
    kad::{
        record::{store::MemoryStore, Key},
        Kademlia, Quorum, Record,
    },
};

pub fn handle_stdin_line(
    kademlia: &mut Kademlia<MemoryStore>,
    line: String,
) {
}
