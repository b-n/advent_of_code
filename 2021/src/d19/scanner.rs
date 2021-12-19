use std::str::FromStr;
use std::collections::HashSet;
use super::beacon::{Beacon, Plane, Plane::*};

#[derive(Debug)]
pub struct Scanner {
    pub name: String,
    beacons: Vec<Beacon>,
    vectors: HashSet<Beacon>,
    rotation: usize,
    pub matches: Vec<String>,
    can_rotate: bool,
}

impl FromStr for Scanner {
    type Err = std::num::ParseIntError;
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut parts = input.split("\n")
            .filter(|x| !x.is_empty());

        let name = String::from(parts.next().unwrap());

        let beacons = parts
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<Beacon>().unwrap())
            .collect::<Vec<Beacon>>();

        Ok(Self::new(name, beacons))
    }
}

impl Scanner {
    fn new(name: String, beacons: Vec<Beacon>) -> Self {
        let mut res = Self {
            name,
            beacons,
            vectors: HashSet::new(),
            rotation: 0,
            matches: vec![],
            can_rotate: true,
        };
        res.calculate_vectors();
        res
    }

    fn calculate_vectors(&mut self) {
        self.vectors.clear();
        for i in 0..self.beacons.len() {
            for j in 0..self.beacons.len() {
                if i != j {
                    self.vectors.insert(self.beacons[i]-self.beacons[j]);
                }
            }
        }
    }
    
    pub fn has_match(&mut self, other: &Self) -> Option<bool> {
        Some((&self.vectors & &other.vectors).len() / 2 >= 12)
    }

    fn rotate_beacons(&mut self, plane: &Plane) {
         for b in self.beacons.iter_mut() {
            b.rotate(plane);
        }
    } 

    pub fn rotate(&mut self) -> Option<()> {
        if !self.can_rotate {
            return None;
        }

        self.rotate_beacons(&XY);
        match self.rotation {
            4 => { self.rotate_beacons(&XZ) },
            8 => { self.rotate_beacons(&YZ) },
            12 => { self.rotate_beacons(&XZ) },
            16 => { self.rotate_beacons(&YZ) },
            20 => { self.rotate_beacons(&XZ) },
            24 => { self.rotate_beacons(&YZ) }, //reset
            _ => (),
        }

        self.calculate_vectors();

        if self.rotation >= 24 {
            self.rotation = 0;
            return None;
        }

        self.rotation += 1;
        
        Some(())
    }

    pub fn add_matches(&mut self, name: &String) {
        self.matches.push(name.clone());
    }

    pub fn stop_rotation(&mut self) {
        self.can_rotate = false; 
    }
}
