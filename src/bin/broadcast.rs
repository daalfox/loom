use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

use loom::{request::Request, response::Response, Node, NodeId};
use serde::{Deserialize, Serialize};
use serde_json::{de, json};

fn main() -> anyhow::Result<()> {
    let mut node = BroadcastNode::init();

    let lines = stdin().lines();
    for line in lines {
        let request: Request<Req> = de::from_str(&line.unwrap())?;
        let response = node.handle(request);

        println!("{}", json!(response));
    }

    Ok(())
}

#[allow(dead_code)]
struct BroadcastNode {
    node_id: NodeId,
    node_ids: Vec<NodeId>,
    topology: HashMap<NodeId, Vec<NodeId>>,
    messages: HashSet<usize>,
}

impl Node for BroadcastNode {
    fn from_init(node_id: NodeId, node_ids: Vec<NodeId>) -> Self {
        BroadcastNode {
            node_id,
            node_ids,
            topology: HashMap::new(),
            messages: HashSet::new(),
        }
    }
}

impl BroadcastNode {
    fn handle(&mut self, request: Request<Req>) -> Response<Res> {
        match request.body.payload {
            Req::Broadcast { message } => {
                self.messages.insert(message);
                request.into()
            }
            Req::Read => {
                let mut response: Response<Res> = request.into();
                response.body.payload = Res::ReadOk {
                    messages: self.messages.clone(),
                };
                response
            }
            Req::Topology { ref topology } => {
                self.topology = topology.clone();
                request.into()
            }
        }
    }
}

#[derive(Debug, Deserialize)]
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
        }
    }
}
