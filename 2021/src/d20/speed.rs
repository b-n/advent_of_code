use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub fn run() {
    let path = Path::new("./input/20");

    println!("Part 1: {}", p01(path).unwrap());
    println!("Part 2: {}", p02(path).unwrap());
}

type Point = (i64, i64);
struct Grid {
    points: HashMap<Point, bool>,
    extent_x: (i64, i64),
    extent_y: (i64, i64),
    step: usize,
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut extent_x = (0, 0);
        let points = input
            .split("\n")
            .filter(|x| !x.is_empty())
            .enumerate()
            .flat_map(|(y, row)| {
                if extent_x.1 == 0 {
                    extent_x.1 = row.len() as i64;
                }
                (0..row.len()).map(move |i| match &row[i..=i] {
                    "#" => ((y as i64, i as i64), true),
                    _ => ((y as i64, i as i64), false),
                })
            })
            .collect::<HashMap<(i64, i64), bool>>();
        let extent_y = (0, points.len() as i64 / extent_x.1);
        Ok(Self {
            extent_x,
            extent_y,
            step: 0,
            points,
        })
    }
}

impl Grid {
    fn step(&mut self, dict: &str) -> Option<()> {
        let new_infinite_points = &dict[((self.step + 1) % 2)..=((self.step + 1) % 2)] == "#";

        self.extent_y = (self.extent_y.0 - 1, self.extent_y.1 + 1);
        self.extent_x = (self.extent_x.0 - 1, self.extent_x.1 + 1);

        let mut flipped = HashSet::new();
        let to_eval = (self.extent_y.0..self.extent_y.1)
            .flat_map(|y| (self.extent_x.0..self.extent_x.1).map(move |x| (y, x)));

        for (y, x) in to_eval {
            let mut pos: usize = 0;
            for y2 in -1..=1 {
                for x2 in -1..=1 {
                    pos <<= 1;
                    let to_check = &(y + y2, x + x2);

                    let current = if self.points.contains_key(to_check) {
                        *self.points.get(to_check)?
                    } else {
                        new_infinite_points
                    };

                    let is_flipped = flipped.contains(to_check);
                    
                    if current ^ is_flipped {
                        pos |= 1
                    }
                }
            }

            let next = &dict[pos..=pos] == "#";
            if self.points.contains_key(&(y, x)) {
                let g = self.points.get_mut(&(y, x))?;
                if *g != next {
                    flipped.insert((y, x));
                }
                *g = next;
            } else {
                if next != new_infinite_points {
                    flipped.insert((y, x));
                }
                self.points.insert((y, x), next);
            }
        }
        self.step += 1;
        Some(())
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in self.extent_y.0..self.extent_y.1 {
            for x in self.extent_x.0..self.extent_x.1 {
                if *self.points.get(&(y,x)).unwrap() {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    fn count(&self) -> Option<usize> {
        Some(self.points.values().filter(|&x| *x).count())
    }
}

fn p01(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;
    let mut parts = raw_input.split("\n\n");

    let dict = parts.next().unwrap();
    let mut grid = parts.next().unwrap().parse::<Grid>().ok()?;

    let steps = 2;
    for _ in 0..steps {
        grid.step(dict)?;
    }

    grid.count()
}

fn p02(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;
    let mut parts = raw_input.split("\n\n");

    let dict = parts.next().unwrap();
    let mut grid = parts.next().unwrap().parse::<Grid>().ok()?;

    let steps = 50;
    for _ in 0..steps {
        grid.step(dict)?;
    }

    grid.count()
}
