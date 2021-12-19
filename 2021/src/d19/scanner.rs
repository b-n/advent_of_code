use std::str::FromStr;
use std::collections::HashSet;
use super::beacon::Beacon;

#[derive(Debug)]
pub struct Scanner {
    name: String,
    pings: Vec<Beacon>,
    vectors: HashSet<Beacon>,
    rotation: usize,
}

impl FromStr for Scanner {
    type Err = std::num::ParseIntError;
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut parts = input.split("\n")
            .filter(|x| !x.is_empty());

        let name = String::from(parts.next().unwrap());

        let pings = parts
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<Beacon>().unwrap())
            .collect::<Vec<Beacon>>();

        Ok(Self::new(name, pings))
    }
}

impl Scanner {
    fn new(name: String, beacons: Vec<Beacon>) -> Self {

        let mut vectors = HashSet::new(); 
        for i in 0..beacons.len() {
            for j in 0..beacons.len() {
                if i != j {
                    vectors.insert(beacons[i]-beacons[j]);
                }
            }
        }

        Self {
            name,
            pings: beacons,
            vectors,
            rotation: 0
        }
    }
    pub fn has_match(&mut self, other: &Self) -> Option<bool> {
        Some(self.vectors.union(&other.vectors).count() >= 12)
    }
}
