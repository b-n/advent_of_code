use super::node::Node;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Eq)]
pub struct NodePath {
    pub nodes: Vec<String>,
    hash: String,
}

impl NodePath {
    pub fn new(nodes: Vec<String>) -> Self {
        let hash = nodes.join(",");
        Self { nodes, hash }
    }

    pub fn from(other: &Self, next: &Node) -> Self {
        Self::new(
            other
                .nodes
                .clone()
                .into_iter()
                .chain([next.name.clone()].into_iter())
                .map(|x| x.clone())
                .collect::<Vec<String>>(),
        )
    }

    pub fn count_node(&self, node: &Node) -> usize {
        self.hash.matches(&node.name).count()
    }
}

impl fmt::Display for NodePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.hash)
    }
}

impl PartialEq for NodePath {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Hash for NodePath {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.hash.hash(state);
    }
}
