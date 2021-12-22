use std::collections::{HashMap, HashSet, BTreeSet};
use std::fs;
use std::path::Path;

pub fn run() {
    let path = Path::new("./input/22");

    //println!("Part 1: {}", p01(path).unwrap());
    println!("Part 2: {}", p02(path).unwrap());
}

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Point3d {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
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

        Ok(Self { p1, p2, state, init })
    }
}

impl Cuboid {
    pub fn iter_points(&self) -> impl Iterator<Item = Point3d> + '_ {
        (self.p1.x..=self.p2.x).flat_map(move |x| {
            (self.p1.y..=self.p2.y)
                .flat_map(move |y| (self.p1.z..=self.p2.z).map(move |z| Point3d { x, y, z }))
        })
    }

    pub fn contains(&self, other: &Self) -> bool{
        other.p1.x >= self.p1.x && other.p1.x <= self.p2.x
        && other.p1.y >= self.p1.y && other.p1.y <= self.p2.y
        && other.p1.z >= self.p1.z && other.p1.z <= self.p2.z
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
        if !c.init { continue }
        for p in c.iter_points() {
            match c.state {
                true => chart.insert(p),
                false => chart.remove(&p),
            };
        }
    }

    Some(chart.len())
}

fn p02(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;

    let cuboids = raw_input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<Cuboid>().unwrap())
        .collect::<Vec<Cuboid>>();


    // What we know, the number of on/off bits is (x.1 - x.0) * (y.1 - y.0) * (z.1 -z.0)
    // if an existing cube is contained entirely inside a new cube, then we can kill the existing
    // The new cube is always taken in it's entirety
    //   - We can drop off cubes, after we cut the existing cubes
    // If any existing cubes are in the way, we need to cut them
    //   - new cube intersects a face of an existing cube, that cube is shrunk on that face
    //   - new cube protrudes into the existing cube, existing cube is split
    //   - the point of intersection determines the amount of new cubes
    //   - At most, the existing cube would be split into 26 cubes (where the new cube fits
    //     entirely into an existing cube
    // In the end, we just go through all cubes and sum
    //
    // What we need:
    //   - An contains and/or intersection function (so we know if we need to do anything)
    //   - A splitting type function

    Some(0)
}
