use crate::manet::node::NodeId;

#[derive(Clone, Debug)]
pub struct Link {
    source: NodeId,
    target: NodeId,
    capacity: f64,             // Maximum capacity of the link
    congestion: f64,           // Current congestion level
}

impl Link {
    pub fn new(source: NodeId, target: NodeId, capacity: f64) -> Self {
        Self {
            source,
            target,
            capacity,
            congestion: 0.0,
        }
    }
}