use core::panic;
use std::{
    collections::{HashMap, HashSet},
    io::stdin,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use loom::{
    request::{self, Request},
    response::Response,
    Node, NodeId,
};
use serde::{Deserialize, Serialize};
use serde_json::{de, json};

fn main() -> anyhow::Result<()> {
    let node = Arc::new(Mutex::new(BroadcastNode::init()));

    let gossip_node = node.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(150));
        gossip_node.lock().unwrap().gossip();
    });

    let lines = stdin().lines();
    for line in lines {
        let request: Request<Req> = de::from_str(&line.unwrap())?;

        if let Some(response) = node.lock().unwrap().handle(request) {
            println!("{}", json!(response));
        }
    }

    Ok(())
}

#[allow(dead_code)]
struct BroadcastNode {
    node_id: NodeId,
    node_ids: Vec<NodeId>,
    topology: HashMap<NodeId, Vec<NodeId>>,
    messages: HashSet<usize>,
    known: HashMap<NodeId, HashSet<usize>>,
}

impl Node for BroadcastNode {
    fn from_init(node_id: NodeId, node_ids: Vec<NodeId>) -> Self {
        let mut known = HashMap::new();
        for node in node_ids.iter() {
            known.insert(node.clone(), HashSet::new());
        }
        BroadcastNode {
            node_id,
            node_ids,
            topology: HashMap::new(),
            messages: HashSet::new(),
            known,
        }
    }
}

impl BroadcastNode {
    fn handle(&mut self, request: Request<Req>) -> Option<Response<Res>> {
        match request.body.payload {
            Req::Broadcast { message } => {
                self.messages.insert(message);
                Some(request.into())
            }
            Req::Read => {
                let mut response: Response<Res> = request.into();
                response.body.payload = Res::ReadOk {
                    messages: self.messages.clone(),
                };
                Some(response)
            }
            Req::Topology { .. } => {
                self.topology = star_topology(&self.node_ids);
                Some(request.into())
            }
            Req::Gossip { ref messages } => {
                self.messages.extend(messages);
                self.known.get_mut(&request.src).unwrap().extend(messages);
                None
            }
        }
    }
    fn gossip(&self) {
        let messages = self.messages.clone();
        let neighborhood = self
            .topology
            .get(&self.node_id)
            .unwrap_or_else(|| panic!("{} has no topology", &self.node_id));
        for n in neighborhood {
            let known_to_n = self
                .known
                .get(n)
                .unwrap_or_else(|| panic!("{n} is an unknown node"));
            let unknown_to_n = messages.difference(known_to_n).copied().collect();

            let req = Request {
                src: self.node_id.to_string(),
                dest: n.to_string(),
                body: request::Body {
                    id: None,
                    payload: Req::Gossip {
                        messages: unknown_to_n,
                    },
                },
            };

            println!("{}", json!(req));
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
enum Req {
    Broadcast {
        message: usize,
    },
    Read,
    Topology {
        topology: HashMap<NodeId, Vec<NodeId>>,
    },
    Gossip {
        messages: HashSet<usize>,
    },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
enum Res {
    BroadcastOk,
    ReadOk { messages: HashSet<usize> },
    TopologyOk,
}
impl From<Req> for Res {
    fn from(value: Req) -> Self {
        match value {
            Req::Broadcast { .. } => Self::BroadcastOk,
            Req::Read => Self::ReadOk {
                messages: HashSet::new(),
            },
            Req::Topology { .. } => Self::TopologyOk,
            Req::Gossip { .. } => panic!("can't reply to gossip messages"),
        }
    }
}

/// make a star topology from the given `nodes`
fn star_topology(nodes: &[NodeId]) -> HashMap<NodeId, Vec<NodeId>> {
    let mut result = HashMap::new();
    result.insert(nodes[0].clone(), nodes[1..].to_vec());
    for node in nodes[1..].iter() {
        result.insert(node.to_string(), nodes[0..1].to_vec());
    }
    result
}
