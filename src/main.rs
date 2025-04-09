mod block;
mod blockchain;
mod data;
mod mempool;
mod network;
mod node;
mod post;
mod randomized_election;
mod transaction;
mod utils;

use blockchain::Blockchain;
use data::Data;
use libp2p::{gossipsub, mdns, swarm::SwarmEvent};
use mempool::MemPoolRequest;
use network::MyBehaviourEvent;
use node::Node;
use transaction::*;

use futures::stream::StreamExt;
use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fs::{self, File},
    io::Write,
    panic,
};

use tokio::{io, io::AsyncBufReadExt, select, time, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (mut swarm, topic) = network::setup_p2p_network()?;

    let node = Node::new();
    let mut blockchain = Blockchain::new_with_genesis_block();
    let mut stdin = io::BufReader::new(io::stdin()).lines();
    let mut broadcast_timer = time::interval(Duration::from_secs(2));
    let mut mine_timer = time::interval(Duration::from_secs(2));
    let mut validate_timer = time::interval(Duration::from_secs(10));
    let mut serving_q: VecDeque<String> = VecDeque::new();
    let mut mempool: VecDeque<MemPoolRequest> = VecDeque::new();
    let mut set_of_nodes: HashSet<String> = HashSet::new();
    set_of_nodes.insert(node.id.to_string());

    loop {
        select! {
            _ = validate_timer.tick() => {
                for (request_id, list) in &blockchain.stored {
                    if !list.contains(&node.id) {
                        continue;
                    }
                }
            }

            _ = broadcast_timer.tick() => {
                Data::broadcast(&node, &blockchain, &mut swarm, &topic).inspect_err(|e| println!("{e}")).ok();
                if let Some(request) = mempool.front() {
                    Data::broadcast(&node, &request, &mut swarm, &topic).inspect_err(|e| println!("{e}")).ok();
                }
                if let Some(request_id) = serving_q.front() {
                    let stx = ServeFileTx{
                        request_id: request_id.to_string(),
                        file_content: fs::read(request_id)?
                    };
                    Data::broadcast(&node, &stx, &mut swarm, &topic).inspect_err(|e| println!("{e}")).ok();
                }
            }

            _ = mine_timer.tick() => {
                if let Some(request) = mempool.front() {
                    request.mine(&node, &mut blockchain, set_of_nodes.len()).inspect_err(|e| println!("{e}")).ok();
                    mempool.pop_front();
                }
            }

            Ok(Some(line)) = stdin.next_line() => {
                if line[0..3] == *"GET" {
                    let query = QueryTx {
                        request_id: line[4..].to_string(),
                    };
                    Data::broadcast(&node, &query, &mut swarm, &topic).inspect_err(|e| println!("{e}")).ok();
                } else if let Some(request) = {
                    MemPoolRequest::new(node.id.to_string(), &line)
                        .inspect_err(|e| println!("{e}"))
                        .ok()
                } {
                    Data::broadcast(&node, &request, &mut swarm, &topic)
                        .inspect_err(|e| println!("{e}"))
                        .ok();
                    println!("Request id: {}", request.request_id);
                    mempool.push_back(request);
                }
            }

            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, _) in list {
                        // println!("+++ New peer discovered");
                        set_of_nodes.insert(peer_id.to_string());
                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                }
                SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                    for (peer_id, _) in list {
                        // println!("--- Peer expired");
                        set_of_nodes.remove(&peer_id.to_string());
                        swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                    }
                }
                SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                    message,
                    ..
                })) => {
                    if let Err(e) = (|| -> Result<(), Box<dyn Error>> {
                        let data = serde_json::from_slice::<Data>(&message.data)?;

                        if !data.verify()? {
                            return Err("cannot verify received data".into());
                        }

                        let data = data.data;
                        if let Ok(received_blockchain) = serde_json::from_slice::<Blockchain>(&data) {
                            if received_blockchain.verify() {
                                blockchain.update(received_blockchain);
                            }
                        } else if let Ok(received_request) = serde_json::from_slice::<MemPoolRequest>(&data) {
                            mempool.push_back(received_request);
                        }
                        else if let Ok(received_file) = serde_json::from_slice::<ServeFileTx>(&data) {
                            let mut fp = File::create(received_file.request_id.to_string() + "_rec")?;
                            fp.write_all(&received_file.file_content)?;
                        }
                        else if let Ok(received_query) = serde_json::from_slice::<QueryTx>(&data) {
                            if let Some(nodeid) = blockchain.stored.get(&received_query.request_id) {
                                if !nodeid.contains(&node.id) {
                                    return Err("queried file is not stored by me".into());
                                }
                                serving_q.push_back(received_query.request_id);
                            }
                        }
                         else {
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
