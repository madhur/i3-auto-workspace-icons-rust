use std::hash::Hash;
use std::{
    hash::Hasher,
};
use swayipc::{Node, NodeType};


#[derive(Debug, Clone)]
pub struct I3Node {
    id: i64,
    name: Option<String>,
    pub node_type: NodeType,
    pub nodes: Vec<I3Node>,
}

impl PartialEq for I3Node {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

impl Eq for I3Node {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Hash for I3Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl I3Node {
    pub fn new(node: Node) -> Self {
        let mut nodes: Vec<I3Node> = Vec::new();
        let node_type = node.node_type;
        let name = node.name;

        for child_node in node.nodes {
            nodes.push(I3Node::new(child_node));
        }
        for child_node in node.floating_nodes {
            nodes.push(I3Node::new(child_node));
        }

        return I3Node {
            id: node.id,
            name: name,
            node_type: node_type,
            nodes: nodes
        };
    }
}
