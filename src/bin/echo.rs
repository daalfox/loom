use std::io::stdin;

use loom::{request::Request, response::Response, Node};
use serde::{Deserialize, Serialize};
use serde_json::{de, json};

fn main() -> anyhow::Result<()> {
    let node = EchoNode::init();

    let lines = stdin().lines();
    for line in lines {
        let request: Request<Echo> = de::from_str(&line.unwrap())?;
        let response = node.handle(request);

        println!("{}", json!(response));
    }

    Ok(())
}

#[allow(dead_code)]
struct EchoNode {
    node_id: String,
    neighbors: Vec<String>,
}

impl Node for EchoNode {
    fn from_init(node_id: String, neighbors: Vec<String>) -> Self {
        EchoNode { node_id, neighbors }
    }
}

impl EchoNode {
    fn handle(&self, request: Request<Echo>) -> Response<EchoOk> {
        request.into()
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename = "echo")]
struct Echo {
    echo: String,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename = "echo_ok")]
struct EchoOk {
    echo: String,
}
impl From<Echo> for EchoOk {
    fn from(value: Echo) -> Self {
        Self { echo: value.echo }
    }
}
