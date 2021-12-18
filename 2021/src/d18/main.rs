use std::fs;
use std::fmt;
use std::path::Path;
use std::str::FromStr;
use std::collections::HashMap;

// Learnings
//
// Rust Enjoyment Factor

pub fn run() {
    let path = Path::new("./input/18_smol_example");

    println!("Part 1: {}", p01(path).unwrap());
    //println!("Part 2: {}", p02(path).unwrap());
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Side {
    Left,
    Right,
}

#[derive(Debug)]
struct Snail {
    depth: usize,
    snails: HashMap<Side, Snail>,
    values: (usize, usize),
}

impl Clone for Snail {
    fn clone(&self) -> Self {
        let mut cloned_snails = HashMap::new();
        for (side, snail) in self.snails.iter() {
            cloned_snails.insert(*side, snail.clone());
        }
        Self {
            values: self.values,
            depth: self.depth,
            snails: cloned_snails,
        }
    }
}

impl FromStr for Snail {
    type Err = std::num::ParseIntError;
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut stack = 0;
        let mut i = 0;
        while i < input.len() {
            let cmp = &input[i..=i];
            if cmp == "[" {
                stack += 1;
            } else if cmp == "]" {
                stack -= 1;
            } else if cmp == "," && stack == 1 {
                break; 
            }
            i += 1;
        };

        let mut values = (0,0);
        let mut snails = HashMap::new();

        let left = &input[1..i];
        if left.len() == 1 {
            values.0 = left.parse::<usize>()?; 
        } else {
            snails.insert(Side::Left, left.parse::<Snail>()?);
        }

        let right = &input[i+1..input.len()-1];
        if right.len() == 1 {
            values.1 = right.parse::<usize>()?;
        } else {
            snails.insert(Side::Right, right.parse::<Snail>()?);
        }
 
        Ok(Self { depth: 0, values, snails })
    }
}

impl fmt::Display for Snail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.has(&Side::Left) && self.has(&Side::Right) {
            write!(f, "[{},{}]", self.get(&Side::Left).unwrap(), self.get(&Side::Right).unwrap())
        } else if self.has(&Side::Left) {
            write!(f, "[{},{}]", self.get(&Side::Left).unwrap(), self.values.1)
        } else if self.has(&Side::Right) {
            write!(f, "[{},{}]", self.values.0, self.get(&Side::Right).unwrap())
        } else {
            write!(f, "[{},{}]", self.values.0, self.values.1)
        }
    }
}

impl Snail {
    pub fn combine(left: &Snail, right: &Snail) -> Self {
        let mut snails = HashMap::new();
        snails.insert(Side::Left, left.clone());
        snails.insert(Side::Right, right.clone());
        Self {
            depth: 0,
            snails,
            values: (0,0),
        }
    }

    fn set_depth(&mut self, depth: usize) {
        self.depth = depth;
        for (_, s) in self.snails.iter_mut() {
            s.set_depth(depth + 1);
        }
    }

    fn has(&self, s: &Side) -> bool {
        self.snails.contains_key(s)
    }

    fn get(&self, s: &Side) -> Option<&Snail> {
        self.snails.get(s)
    }
    
    fn get_mut(&mut self, s: &Side) -> Option<&mut Snail> {
        self.snails.get_mut(s)
    }

    fn remove(&mut self, s: &Side) -> Option<Snail>{
        self.snails.remove(&s)
    }
    
    fn is_zero(&self) -> bool {
        self.snails.is_empty() && self.values == (0,0)
    }

    fn add(&mut self, value: usize, side: &Side) -> Option<usize> {
        // we want to add value to side
        Some(
            match side {
                Side::Left => {
                    if self.has(&Side::Left) {
                        self.get_mut(&Side::Left)?.add(value, side)?
                    } else {
                        self.values.0 += value;
                        0
                    }
                },
                Side::Right => {
                    if self.has(&Side::Right) {
                        self.get_mut(&Side::Right)?.add(value, side)?
                    } else {
                        self.values.1 += value;
                        0
                    }
                },
            }
        )
    }

    fn prune(&mut self, side: &Side) -> Option<bool> {
        if self.has(side) && self.get(side)?.is_zero() {
            self.remove(side)?;
        }
        Some(true)
    }

    pub fn explode(&mut self) -> Option<(bool, (usize, usize))> {
        if !self.has(&Side::Left) && !self.has(&Side::Right) && self.depth >= 4 {
            let res = self.values;
            self.values = (0, 0);
            return Some((true, res));
        }

        if self.has(&Side::Left) {
            let res = self.get_mut(&Side::Left)?.explode()?;
            let mut values = res.1;

            if res.0 {
                self.prune(&Side::Left)?;
                if values.1 > 0 {
                    if !self.has(&Side::Right) {
                        self.values.1 += values.1;
                        values.1 = 0;
                    } else {
                        values.1 = self.get_mut(&Side::Right)?.add(values.1, &Side::Left)?;
                    } 
                }
                return Some((res.0, values))
            }
        }

        if self.has(&Side::Right) {
            let res = self.get_mut(&Side::Right)?.explode()?;
            let mut values = res.1;

            if res.0 {
                self.prune(&Side::Right)?;
                if values.0 > 0 {
                    if !self.has(&Side::Left) {
                        self.values.0 += values.0;
                        values.0 = 0;
                    } else {
                        values.0 = self.get_mut(&Side::Left)?.add(values.0, &Side::Right)?;
                    } 
                }
                return Some((res.0, values)) 
            }
        }

        Some((false, (0,0)))
    }

    pub fn split(&mut self) -> Option<bool> {
        if self.has(&Side::Left) {
            if self.get_mut(&Side::Left)?.split()? {
                return Some(true)
            }
        }
        if self.values.0 >= 10 {
            let values = (self.values.0 / 2, (self.values.0 + 1) / 2);
            self.snails.insert(Side::Left, Snail {
                depth: self.depth + 1,
                values,
                snails: HashMap::new()
            });
            self.values.0 = 0;
            return Some(true)
        }
        if self.has(&Side::Right) {
            if self.get_mut(&Side::Right)?.split()? {
                return Some(true)
            }
        }
        if self.values.1 >= 10 {
            let values = (self.values.1 / 2, (self.values.1 + 1) / 2);
            self.snails.insert(Side::Right, Snail {
                depth: self.depth + 1,
                values,
                snails: HashMap::new()
            });
            self.values.1 = 0;
            return Some(true)
        }

        Some(false)
    }

    fn magnitude(&self) -> Option<usize> {
        let left = if self.has(&Side::Left) {
            self.get(&Side::Left)?.magnitude()?
        } else {
            self.values.0
        } * 3;
        let right = if self.has(&Side::Right) {
            self.get(&Side::Right)?.magnitude()?
        } else {
            self.values.1
        } * 2;

        Some(left + right)
    }
}


fn p01(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;

    let snails = raw_input.split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let mut snail = x.parse::<Snail>().unwrap();
            snail.set_depth(0);
            snail
        })
        .collect::<Vec<Snail>>();

    let mut combined_snail = snails[0].clone();

    for i in 1..snails.len() {
        combined_snail = Snail::combine(&combined_snail, &snails[i].clone());
        combined_snail.set_depth(0);
        let mut changed = true;
        while changed {
            let (exploded, _) = combined_snail.explode()?;
            changed = exploded;
            if changed {
                //println!("exploded");
            }
            if !changed {
                let split = combined_snail.split()?;
                changed = split;
                if changed {
                    //println!("split");
                }
            }
        }
        println!("{}", combined_snail);
    }
    //println!("{}", combined_snail);

    Some(combined_snail.magnitude()?)
}
