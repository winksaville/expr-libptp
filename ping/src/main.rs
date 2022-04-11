use futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use libp2p::{identity, PeerId, Swarm, Multiaddr};
use libp2p::ping::{Ping, PingConfig};
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    let transport = libp2p::development_transport(local_key).await?;
    println!("transport: {:#?}", transport);

    // Create a Ping "protocol" or "behaviour"?
    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));
    println!("behaviour created");

    // Create a "Swarm/Switch"
    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);
    println!("swarm created");

    // Listen on the a all interfaces and a randomly assigned port
    let listen_addr = "/ip4/0.0.0.0/tcp/0";
    swarm.listen_on(listen_addr.parse()?)?;
    println!("swarm listening on {}", listen_addr);

    println!("std::env::args len: {}", std::env::args().len());
    println!("std::env::args {:?}", std::env::args());

    // Expect one paramter
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {}", addr);
    }

    loop {
        println!("Looper waiting on some event");
        let swarm_event = swarm.select_next_some().await;
        match swarm_event {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on {:?}", address);
            }
            SwarmEvent::Behaviour(event) => {
                println!("Behaviour event: {:?}", event);
            }
            _ => {
                println!("Ignoring swarm_event: {:?}", swarm_event)
            }
        }
    }
}
