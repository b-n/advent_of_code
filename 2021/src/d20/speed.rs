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
            for i in 0..9 {
                pos <<= 1;
                let to_check = &(y - 1 + i / 3, x - 1 + i % 3);

                let current = match self.points.get(to_check) {
                    Some(value) => value,
                    None => &new_infinite_points,
                };

                if match (flipped.get(to_check), current) {
                    (Some(_), false) => true,
                    (None, true) => true,
                    _ => false,
                } {
                    pos |= 1
                }
            }

            let next = &dict[pos..=pos] == "#";
            let current = self.points.entry((y, x)).or_insert(new_infinite_points);
            if &next != current {
                flipped.insert((y, x));
                *current = next;
            }
        }
        self.step += 1;
        Some(())
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in self.extent_y.0..self.extent_y.1 {
            for x in self.extent_x.0..self.extent_x.1 {
                if *self.points.get(&(y, x)).unwrap() {
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
