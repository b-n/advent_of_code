use std::str::FromStr;
use std::collections::{HashSet, HashMap};
use super::beacon::{Beacon, Plane, Plane::*};

#[derive(Debug)]
pub struct Scanner {
    pub id: usize,
    beacons: Vec<Beacon>,
    pub vectors: HashMap<Beacon, Beacon>,
    rotation: usize,
    pub matches: Vec<usize>,
    can_rotate: bool,
    pub center: Beacon,
    pub has_center: bool,
}

impl FromStr for Scanner {
    type Err = std::num::ParseIntError;
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut parts = input.split("\n")
            .filter(|x| !x.is_empty());

        let title = parts.next().unwrap()[12..15].split(" ").next().unwrap();
        let id = title.parse::<usize>().unwrap();

        let beacons = parts
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<Beacon>().unwrap())
            .collect::<Vec<Beacon>>();

        Ok(Self::new(id, beacons))
    }
}

impl Scanner {
    fn new(id: usize, beacons: Vec<Beacon>) -> Self {
        let mut res = Self {
            id,
            beacons,
            vectors: HashMap::new(),
            rotation: 0,
            matches: vec![],
            can_rotate: true,
            center: Beacon { x: 0, y: 0, z: 0 },
            has_center: false
        };
        res.calculate_vectors();
        res
    }

    pub fn set_center(&mut self, center: Beacon) {
        self.center = center;
        self.has_center = true;
    }

    fn calculate_vectors(&mut self) {
        self.vectors.clear();
        for i in 0..self.beacons.len() {
            for j in 0..self.beacons.len() {
                if i != j {
                    let b1 = self.beacons[i];
                    let b2 = self.beacons[j];
                    self.vectors.insert(b1 - b2, b1);
                }
            }
        }
    }

    pub fn matching_vectors(&self, other: &Self) -> HashSet<Beacon> {
        &self.vectors.keys().cloned().collect::<HashSet<Beacon>>() &
            &other.vectors.keys().cloned().collect::<HashSet<Beacon>>()
    }
    
    pub fn has_match(&self, other: &Self) -> Option<bool> {
        Some(self.matching_vectors(other).len() / 2 >= 12)
    }

    fn rotate_beacons(&mut self, plane: &Plane) {
         for b in self.beacons.iter_mut() {
            b.rotate(plane);
        }
    } 

    pub fn beacons(&self) -> HashSet<Beacon> {
        self.beacons.iter()
            .map(|x| *x + self.center)
            .collect()
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

    pub fn add_matches(&mut self, id: usize) {
        self.matches.push(id);
    }

    pub fn stop_rotation(&mut self) {
        self.can_rotate = false; 
    }
}
