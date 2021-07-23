impl NetworkBehaviourEventProcess<SwarmEvent<MerosBehavior, MerosNetError>> for MerosBehavior {
    /// Upon a swarm event
    fn inject_event(&mut self, event: SwarmEvent<MerosBehavior, MerosNetError>) {
        match event {
            SwarmEvent::ConnectionEstablished {
                peer_id,
                endpoint,
                num_established,
            } => {
                println!("peer joined: {:?}", peer_id);
            }

            SwarmEvent::ConnectionClosed {
                peer_id,
                endpoint,
                num_established,
                cause,
            } => println!("peer left: {:?}", peer_id),

            _ => println!("swarm event"),
        }
    }
}
