use crate::{common, GeneralError};
use libp2p::{identity, PeerId};
use std::{error::Error, fs, io::Write, path::Path};

pub struct Identity {
    /// The node's libp2p ed25519 keypair
    pub keypair: identity::Keypair,

    /// The node's libp2p PeerId (hash of the pubk)
    pub peer_id: PeerId,
}

impl Identity {
    pub fn new(name: &str) -> Result<Self, Box<dyn Error>> {
        let path = Path::new(common::DATADIR).join("identities").join(name);

        // If the identity already exists, load it from disk
        if path.exists() {
            let keypair =
                identity::Keypair::Ed25519(identity::ed25519::Keypair::decode(
                    &mut fs::read(path.join("keypair"))?,
                )?);
            return Ok(Identity {
                peer_id: PeerId::from_public_key(keypair.public()),
                keypair,
            });
        }

        // If it does not, create it and persist it to disk
        let keypair = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from_public_key(keypair.public());
        fs::create_dir_all(&path)?;
        if let identity::Keypair::Ed25519(k) = &keypair {
            fs::File::create(&path.join("keypair"))?.write_all(&k.encode())?;
            return Ok(Identity { keypair, peer_id });
        }
        Err(Box::new(GeneralError {
            details: String::from("error creating identity"),
        }))
    }
}
