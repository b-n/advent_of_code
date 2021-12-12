use std::fmt;

// Today we battled lifetimes - and lost
// - I wanted to store a node reference instead of a string - but rust says no
#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub connections: Vec<String>,
    pub is_smol: bool,
}

impl Node {
    pub fn new(name: String) -> Self {
        let is_smol = name.chars().all(|x| x.is_ascii_lowercase());
        Self { name: name, connections: vec![], is_smol }
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
