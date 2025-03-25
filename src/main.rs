mod block;
mod blockchain;
mod data;
mod mempool;
mod node;
mod randomized_election;
mod transaction;
mod utils;

use block::Block;
use blockchain::Blockchain;
use data::Data;
use mempool::{MemPool, MemPoolRequest};
use node::Node;
use randomized_election::is_elected;
use std::{collections::HashMap, panic};
use tokio::time;

use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    hash::{Hash, Hasher},
    time::Duration,
};

use futures::stream::StreamExt;
use libp2p::{
    PeerId, gossipsub, identity, mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux,
};
use tokio::{io, io::AsyncBufReadExt, select};
use tracing_subscriber::EnvFilter;

// We create a custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
struct MyBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_quic()
        .with_behaviour(|key| {
            // To content-address message, we can take the hash of message and use it as an ID.
            let message_id_fn = |message: &gossipsub::Message| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                gossipsub::MessageId::from(s.finish().to_string())
            };

            // Set a custom gossipsub configuration
            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
                .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message
                // signing)
                .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
                .build()
                .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?; // Temporary hack because `build` does not return a proper `std::error::Error`.

            // build a gossipsub network behaviour
            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )?;

            let mdns =
                mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;
            Ok(MyBehaviour { gossipsub, mdns })
        })?
        .build();

    // Create a Gossipsub topic
    let topic = gossipsub::IdentTopic::new("test-net");
    // subscribes to our topic
    swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

    // Listen on all interfaces and whatever port the OS assigns
    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    let node = match Node::new() {
        Ok(node) => node,
        Err(_) => panic!("cannot make a new node instance"),
    };

    let mut blockchain = Blockchain {
        chain: vec![],
        nodes: vec![],
        stored: HashMap::new(),
        balance: HashMap::new(),
    };

    // self id
    blockchain.nodes.push(node.id.to_string());

    // Geneisis block
    let mut gen_block = Block {
        previous_hash: None,
        tx: None,
        hash: None,
    };
    gen_block.calculate_hash();
    blockchain.chain.push(gen_block);

    let mut mempool = MemPool {
        pending: vec![].into(),
        max_size: 1000,
    };

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    let mut broadcast_timer = time::interval(Duration::from_secs(10));
    let mut mine_timer = time::interval(Duration::from_secs(10));
    loop {
        select! {
            _ = broadcast_timer.tick() => {
                if let Err(e) = (|| -> Result<(), Box<dyn Error>> {
                    let data =  serde_json::to_vec(&blockchain)?;
                    Data::new(node.id.clone().to_string(), data, &node.private_key, node.public_key.clone(), true)?.broadcast(&mut swarm, &topic)?;
                    for request in &mempool.pending {
                        let data =  serde_json::to_vec(&request)?;
                        Data::new(node.id.clone().to_string(), data, &node.private_key, node.public_key.clone(), true)?.broadcast(&mut swarm, &topic)?;
                    }
                    Ok(())
                })() {
                    println!("[!!] {e}");
                }
            }

            _ = mine_timer.tick() => {
                println!("[#] current mempool: {:#?}", mempool);

                if let Err(e) = (|| -> Result<(), Box<dyn Error>> {
                    let req = mempool.get_first()?;
                    if req.node_id == node.id {
                        return Err("Requesting node is same as miner node".into());
                    }

                    if blockchain.search_transaction(&req.request_id) {
                        mempool.pending.pop_front();
                        return Err("Request already served".into());
                    }

                    let mut block = Block {
                        previous_hash: blockchain.chain.last().unwrap().hash.clone(),
                        tx: Some(req.mine(&node.id)),
                        hash: None,
                    };
                    block.calculate_hash();

                    if !is_elected(&node.id, &block.hash.clone().unwrap(), blockchain.chain.len()) {
                        return Err("Not eligible to propose a block".into());
                    }

                    blockchain.add_block(block.clone());
                    blockchain.stored.insert(req.file_hash, node.id.clone());

                    Ok(())
                })() {
                    println!("[!!] {e}");
                }
            }

            Ok(Some(line)) = stdin.next_line() => {
                if let Err(e) = (|| -> Result<(), Box<dyn Error>> {
                    let request = MemPoolRequest::new(node.id.to_string(), &line, 1.0)?;
                    mempool.add(&request);

                    let data = serde_json::to_vec(&request)?;
                    Data::new(node.id.clone().to_string(), data, &node.private_key, node.public_key.clone(), true)?.broadcast(&mut swarm, &topic)?;
                    Ok(())
                })() {
                    println!("[!!] {e}");
                }
            }

            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, _) in list {
                        println!("+++ New peer discovered: {peer_id}");
                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                }
                SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                    for (peer_id, _) in list {
                        println!("--- Peer expired: {peer_id}");
                        swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                    }
                }
                SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                    message,
                    ..
                })) => {
                    if let Err(e) = (|| -> Result<(), Box<dyn Error>> {
                        let data = serde_json::from_slice::<Data>(&message.data)?;
                        if !data.is_signed {
                            // TODO: in future handle this if there is some unsigned data needed to be sent
                            return Ok(());
                        }
                        if !data.verify()? {
                            return Err("cannot verify received data".into());
                        }
                        let data = data.data;
                        if let Ok(received_blockchain) = serde_json::from_slice::<Blockchain>(&data) {
                            //println!("[#] Received Blockchain: {:#?}", received_blockchain);
                            if received_blockchain.verify() {
                                blockchain.update(received_blockchain);
                                //println!("[DEBUG] current chain: {:#?}", blockchain);
                            }
                        } else if let Ok(received_request) = serde_json::from_slice::<MemPoolRequest>(&data) {
                            //println!("[#] Received request: {:#?}", received_request);
                            mempool.add(&received_request);
                        }  else {
                            return Err("invalid received_signed_data".into());
                        }
                        Ok(())
                    })() {
                        println!("[!!] {e}");
                    }
                }
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("[#] Listening on {address}");
                }
                _ => {}
            }
        }
    }
}
