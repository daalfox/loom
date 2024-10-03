use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request<P> {
    pub src: String,
    pub dest: String,
    pub body: Body<P>,
}

#[derive(Debug, Deserialize)]
pub struct Body<P> {
    #[serde(rename = "msg_id")]
    pub id: usize,
    #[serde(flatten)]
    pub payload: P,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
#[serde(rename = "init")]
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}
