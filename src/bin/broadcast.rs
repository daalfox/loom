use std::{collections::HashMap, io::stdin};

use loom::{request::Request, response::Response, Node};
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
    node_id: String,
    neighbors: Vec<String>,
    messages: Vec<usize>,
}

impl Node for BroadcastNode {
    fn from_init(node_id: String, neighbors: Vec<String>) -> Self {
        BroadcastNode {
            node_id,
            neighbors,
            messages: Vec::new(),
        }
    }
}

impl BroadcastNode {
    fn handle(&mut self, request: Request<Req>) -> Response<Res> {
        match request.body.payload {
            Req::Broadcast { message } => {
                self.messages.push(message);
                request.into()
            }
            Req::Read => {
                let mut response: Response<Res> = request.into();
                response.body.payload = Res::ReadOk {
                    messages: self.messages.clone(),
                };
                response
            }
            Req::Topology { .. } => request.into(),
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
        topology: HashMap<String, Vec<String>>,
    },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
enum Res {
    BroadcastOk,
    ReadOk { messages: Vec<usize> },
    TopologyOk,
}
impl From<Req> for Res {
    fn from(value: Req) -> Self {
        match value {
            Req::Broadcast { .. } => Self::BroadcastOk,
            Req::Read => Self::ReadOk {
                messages: Vec::new(),
            },
            Req::Topology { .. } => Self::TopologyOk,
        }
    }
}
