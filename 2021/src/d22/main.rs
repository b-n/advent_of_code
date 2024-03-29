use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

pub fn run() {
    let path = Path::new("./input/22");

    //println!("Part 1: {}", p01(path).unwrap());
    println!("Part 2: {}", p02(path).unwrap());
}

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
struct Point3d {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone, Copy)]
struct Cuboid {
    p1: Point3d,
    p2: Point3d,
    state: bool,
    init: bool,
}

impl std::str::FromStr for Cuboid {
    type Err = std::num::ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut raw_parts = input.split(" ");

        let state = raw_parts.next().unwrap() == "on";

        let poss = raw_parts
            .next()
            .unwrap()
            .split(",")
            .map(|v| {
                let axis = &v[0..=0];
                let mut fr_to = v[2..v.len()].split("..");
                let from = fr_to.next().unwrap().parse::<i64>().unwrap();
                let to = fr_to.next().unwrap().parse::<i64>().unwrap();
                (axis, (from, to))
            })
            .collect::<HashMap<&str, (i64, i64)>>();
        let mut x = *poss.get(&"x").unwrap();
        if x.0 > x.1 {
            x = (x.1, x.0);
        }
        let mut y = *poss.get(&"y").unwrap();
        if y.0 > y.1 {
            y = (y.1, y.0);
        }
        let mut z = *poss.get(&"z").unwrap();
        if z.0 > z.1 {
            z = (z.1, z.0);
        }

        let p1 = Point3d {
            x: x.0,
            y: y.0,
            z: z.0,
        };
        let p2 = Point3d {
            x: x.1,
            y: y.1,
            z: z.1,
        };

        let init = x.0 >= -50 && x.1 <= 50 && y.0 >= -50 && y.1 <= 50 && z.0 >= -50 && z.1 <= 50;

        Ok(Self {
            p1,
            p2,
            state,
            init,
        })
    }
}

impl Cuboid {
    fn new(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> Self {
        Cuboid {
            p1: Point3d {
                x: p1.0,
                y: p1.1,
                z: p1.2,
            },
            p2: Point3d {
                x: p2.0,
                y: p2.1,
                z: p2.2,
            },
            state: true,
            init: false,
        }
    }

    pub fn iter_points(&self) -> impl Iterator<Item = Point3d> + '_ {
        (self.p1.x..=self.p2.x).flat_map(move |x| {
            (self.p1.y..=self.p2.y)
                .flat_map(move |y| (self.p1.z..=self.p2.z).map(move |z| Point3d { x, y, z }))
        })
    }

    pub fn get_cubes(&self, other: &Self) -> Option<Vec<Cuboid>> {
        // do x
        let mut res = vec![];
        let new_x = match get_new_points((self.p1.x, self.p2.x), (other.p1.x, other.p2.x)) {
            (Some(p1), None, Some(rest)) => {
                res.push(Cuboid::new(
                    (p1.0, other.p1.y, other.p1.z),
                    (p1.1, other.p2.y, other.p2.z),
                ));
                rest
            }
            (Some(p1), Some(p2), Some(rest)) => {
                res.push(Cuboid::new(
                    (p1.0, other.p1.y, other.p1.z),
                    (p1.1, other.p2.y, other.p2.z),
                ));
                res.push(Cuboid::new(
                    (p2.0, other.p1.y, other.p1.z),
                    (p2.1, other.p2.y, other.p2.z),
                ));
                rest
            }
            (None, None, Some(rest)) => rest,
            (None, None, None) => return None,
            _ => unreachable!(),
        };
        let new_y = match get_new_points((self.p1.y, self.p2.y), (other.p1.y, other.p2.y)) {
            (Some(p1), None, Some(rest)) => {
                res.push(Cuboid::new(
                    (new_x.0, p1.0, other.p1.z),
                    (new_x.1, p1.1, other.p2.z),
                ));
                rest
            }
            (Some(p1), Some(p2), Some(rest)) => {
                res.push(Cuboid::new(
                    (new_x.0, p1.0, other.p1.z),
                    (new_x.1, p1.1, other.p2.z),
                ));
                res.push(Cuboid::new(
                    (new_x.0, p2.0, other.p1.z),
                    (new_x.1, p2.1, other.p2.z),
                ));
                rest
            }
            (None, None, Some(rest)) => rest,
            (None, None, None) => return None,
            _ => unreachable!(),
        };
        match get_new_points((self.p1.z, self.p2.z), (other.p1.z, other.p2.z)) {
            (Some(p1), None, Some(rest)) => {
                res.push(Cuboid::new(
                    (new_x.0, new_y.0, p1.0),
                    (new_x.1, new_y.1, p1.1),
                ));
                rest
            }
            (Some(p1), Some(p2), Some(rest)) => {
                res.push(Cuboid::new(
                    (new_x.0, new_y.0, p1.0),
                    (new_x.1, new_y.1, p1.1),
                ));
                res.push(Cuboid::new(
                    (new_x.0, new_y.0, p2.0),
                    (new_x.1, new_y.1, p2.1),
                ));
                rest
            }
            (None, None, Some(rest)) => rest,
            (None, None, None) => return None,
            _ => unreachable!(),
        };
        // we don't need to use new_z

        Some(res)
    }

    pub fn size(&self) -> i64 {
        (self.p2.x - self.p1.x + 1) * (self.p2.y - self.p1.y + 1) * (self.p2.z - self.p1.z + 1)
    }
}

//if any point is inside the new cuboid (represented by [ ]), we are intersecting e.g.
//  -[--{-]-}--  left point is inside, then cube = (new right, right)
//  -{--[-}-]--  right point is inside, then cube = (left, new left)
//  -[--{-}-]--  both points are inside (covered by the above) then cube = None;
//or the existing point is outside the bounds of the new point (e.g. existing point
//encompases new point
//  -{--[-]-}-   over the extent, then cubes = (left, new left), (right, new right)

// returns (new1, new2, cut)
pub fn get_new_points(
    new: (i64, i64),
    existing: (i64, i64),
) -> (Option<(i64, i64)>, Option<(i64, i64)>, Option<(i64, i64)>) {
    if new.0 <= existing.0 && new.1 <= existing.1 && new.1 >= existing.0 {
        (
            Some((new.1 + 1, existing.1)),
            None,
            Some((existing.0, new.1)),
        )
    } else if new.0 >= existing.0 && new.0 <= existing.1 && new.1 >= existing.1 {
        (
            Some((existing.0, new.0 - 1)),
            None,
            Some((new.0, existing.1)),
        )
    } else if new.0 <= existing.0 && new.1 >= existing.1 {
        (None, None, Some((existing.0, existing.1)))
    } else if new.0 > existing.0 && new.1 < existing.1 {
        (
            Some((existing.0, new.0 - 1)),
            Some((new.1 + 1, existing.1)),
            Some((new.0, new.1)),
        )
    } else {
        (None, None, None)
    }
}

fn p01(p: &Path) -> Option<usize> {
    // This naive approach eats through 16GB ram on P2 in 1 minute
    let raw_input = fs::read_to_string(p).ok()?;

    let cuboids = raw_input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<Cuboid>().unwrap())
        .collect::<Vec<Cuboid>>();

    let mut chart = HashSet::new();

    for c in cuboids.iter() {
        if !c.init {
            continue;
        }
        for p in c.iter_points() {
            match c.state {
                true => chart.insert(p),
                false => chart.remove(&p),
            };
        }
    }

    Some(chart.len())
}

fn p02(p: &Path) -> Option<i64> {
    let raw_input = fs::read_to_string(p).ok()?;

    let cuboids = raw_input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<Cuboid>().unwrap())
        .collect::<Vec<Cuboid>>();

    let mut map = vec![];

    for cube in cuboids {
        let mut next: Vec<Cuboid> = vec![];
        for existing in map.iter() {
            match cube.get_cubes(&existing) {
                Some(mut cubes) => next.append(&mut cubes),
                None => {
                    if existing.state {
                        next.push(existing.clone());
                    }
                }
            }
        }
        if cube.state {
            next.push(cube);
        }
        map = next;
    }

    let total = map.iter().map(|x| x.size()).sum();

    Some(total)
}
