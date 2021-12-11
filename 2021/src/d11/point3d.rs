use std::fmt;

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct Point3d {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl PartialEq for Point3d {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl fmt::Display for Point3d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{} => {}", self.x, self.y, self.z)
    }
}
