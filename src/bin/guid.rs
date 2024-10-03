use std::io::stdin;

use loom::{request::Request, response::Response, Node};
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
        let mut response: Response<GenerateOk> = request.into();
        response.body.payload.guid =
            Some(format!("{}-{}", self.node_id, response.body.in_reply_to));
        response
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
    guid: Option<String>,
}

impl From<Generate> for GenerateOk {
    fn from(_value: Generate) -> Self {
        Self { guid: None }
    }
}
