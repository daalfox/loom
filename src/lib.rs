use std::io::stdin;

use request::{Init, Request};
use response::{InitOk, Response};
use serde_json::{de, json};

pub mod request;
pub mod response;

pub trait Node {
    fn from_init(node_id: String, node_ids: Vec<String>) -> Self;
    fn init() -> Self
    where
        Self: Sized,
    {
        let mut lines = stdin().lines();
        let first_line = lines.next().unwrap().unwrap();

        let init_msg: Request<Init> =
            de::from_str(&first_line).expect("first message should be init");

        let Init { node_id, node_ids } = init_msg.body.payload.clone();

        let response: Response<InitOk> = init_msg.into();

        println!("{}", json!(response));

        Self::from_init(node_id.to_string(), node_ids)
    }
}
