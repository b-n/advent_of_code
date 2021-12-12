use std::fmt;

// Today we battled lifetimes - and lost
// - I wanted to store a node reference instead of a string - but rust says no
#[derive(Debug, Clone, Eq, Hash)]
pub struct Node {
    pub name: String,
    pub connections: Vec<String>,
    pub is_smol: bool,
    pub is_end: bool,
}

impl Node {
    pub fn new(name: String) -> Self {
        let is_smol = name.chars().all(|x| x.is_ascii_lowercase());
        let is_end = name == "end";
        Self { name: name, connections: vec![], is_smol, is_end }
    }

    pub fn add_connection(&mut self, to: String) {
        self.connections.push(to.clone());
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({}) => {}", self.name, self.is_smol, self.connections.len())
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
