use std::collections::HashMap;
use crate::manet::link::Link;
use crate::manet::node::{Node, NodeId};

pub struct Graph {
    nodes: HashMap<NodeId, Node>,
    links: HashMap<(NodeId, NodeId), Link> // source, target
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            links: HashMap::new(),
        }
    }

    fn add_node(&mut self, id: usize, transmission_limit: f64) {
        let node = Node::new(id, transmission_limit);
        self.nodes.insert(id, node);
    }

    fn add_link(&mut self, source: NodeId, target: NodeId, capacity: f64) {
        let link = Link::new(source, target, capacity);

        if let Some(source_node) = self.nodes.get_mut(&source) {
            source_node.neighbors.insert(target);
        }
        if let Some(target_node) = self.nodes.get_mut(&target) {
            target_node.neighbors.insert(source);
        }

        self.links.insert((source, target), link);
    }
}