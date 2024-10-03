use std::io::stdin;

use loom::{
    request::Request,
    response::{self, Response},
    Node,
};
use serde::{Deserialize, Serialize};
use serde_json::{de, json};

fn main() -> anyhow::Result<()> {
    let node = GuidNode::init();

    let lines = stdin().lines();
    for line in lines {
        let request: Request<Generate> = de::from_str(&line.unwrap())?;
        let response = node.handle(request);

        println!("{}", json!(response));
    }

    Ok(())
}

#[allow(dead_code)]
struct GuidNode {
    node_id: String,
    neighbors: Vec<String>,
}

impl Node for GuidNode {
    fn from_init(node_id: String, neighbors: Vec<String>) -> Self {
        GuidNode { node_id, neighbors }
    }
}

impl GuidNode {
    fn handle(&self, request: Request<Generate>) -> Response<GenerateOk> {
        Response {
            src: request.dest,
            dest: request.src,
            body: response::Body {
                in_reply_to: request.body.id,
                payload: GenerateOk {
                    guid: format!("{}-{}", self.node_id, request.body.id),
                },
            },
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename = "generate")]
struct Generate {}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename = "generate_ok")]
struct GenerateOk {
    #[serde(rename = "id")]
    guid: String,
}
