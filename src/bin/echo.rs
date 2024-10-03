use loom::Node;

fn main() {
    let _echo_node = EchoNode::init();
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
