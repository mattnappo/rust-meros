use libp2p::multiaddr::Multiaddr;
use serde::{Deserialize, Serialize};

/// Necessary metadata representing a node on the network.
/// This struct is mainly used by the primitives::File struct.
/// There will also be some challenges making sure that the
/// data in this structure is always up-to-date.
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeIdentity {
    address: Multiaddr,
    // status: NodeStatus,
    // public_key: PublicKey, // TODO
}

/// The status of a node.
pub enum NodeStatus {
    Online,
    Offline,
    Unknown,
}
