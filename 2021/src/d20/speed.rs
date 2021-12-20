use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::str::FromStr;

pub fn run() {
    let path = Path::new("./input/20");

    println!("Part 1: {}", p01(path).unwrap());
    //println!("Part 2: {}", p02(path).unwrap());
}

type Point = (i64, i64);
struct Grid {
    points: HashMap<Point, bool>,
    extent_x: (i64, i64),
    extent_y: (i64, i64),
    step: usize
}

impl FromStr for Grid {
    type Err = (); 
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut extent_x = (0, 0);
        let points = input.split("\n")
            .filter(|x| !x.is_empty())
            .enumerate()
            .flat_map(|(y, row)| {
                if extent_x.1 == 0 {
                    extent_x.1 = row.len() as i64;
                }
                (0..row.len())
                    .map(move |i| {
                        match &row[i..=i] {
                            "#" => ((y as i64, i as i64), true),
                            _ => ((y as i64, i as i64), false),
                        }
                    })
            })
            .collect::<HashMap<(i64, i64), bool>>();
        let extent_y = (0, points.len() as i64 / extent_x.1);
        Ok(Self { points, extent_x, extent_y, step: 0 })
    }
}

impl Grid {
    fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<'_, (i64, i64), bool> {
        self.points.iter_mut()
    }

    fn step(&self, dict: &str) -> Option<Self> {
        let infinite_points = &dict[(self.step % 2)..=(self.step % 2)] == "#";
        // This is stupid inefficient, but it works. Incoming hot fix later
        let mut next = Self {
            extent_x: (self.extent_x.0 - 1, self.extent_x.1 + 1),
            extent_y: (self.extent_y.0 - 1, self.extent_y.1 + 1),
            step: self.step + 1,
            points: HashMap::new(),
        };

        for ((y, x), g) in next.iter_mut() {
            let mut pos: usize = 0;
            for y2 in -1..=1 {
                for x2 in -1..=1 {
                    pos <<= 1;
                    let to_check = &(y + y2, x + x2);
                    if self.points.contains_key(to_check) && *self.points.get(to_check)? {
                        pos |= 1;
                    } else if !self.points.contains_key(to_check) && !infinite_points {
                        pos |= 1;
                    }
                }
            }
            *g = &dict[pos..=pos] == "#";
        }
        Some(next)
    }

    fn count(&self) -> Option<usize> {
        Some(self.points.values()
        .filter(|&x| *x)
        .count())
    }
}



fn p01(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;
    let mut parts = raw_input.split("\n\n");

    let dict = parts.next().unwrap();
    let mut grid = parts.next().unwrap().parse::<Grid>().ok()?;

    let steps = 2;
    for _ in 0..steps {
        grid = grid.step(dict)?;
    }

    grid.count()
}

//fn p01(p: &Path) -> Option<usize> {
    //let raw_input = fs::read_to_string(p).ok()?;
    //let mut parts = raw_input.split("\n\n");

    //let dict = parts.next().unwrap();
    //let mut grid = parts.next().unwrap().split("\n")
        //.filter(|x| !x.is_empty())
        //.enumerate()
        //.flat_map(|(y, row)| {
            //(0..row.len())
                //.map(move |i| {
                    //match &row[i..=i] {
                        //"#" => ((y as i64, i as i64), true),
                        //_ => ((y as i64, i as i64), false),
                    //}
                //})
        //})
        //.collect::<HashMap<(i64, i64), bool>>();

    //let steps = 2;
    //for i in 0..steps {
        //grid = step_grid(&grid, dict, i)?;
    //}

    //Some(grid.values()
        //.filter(|&x| *x)
        //.count())
//}

//fn p02(p: &Path) -> Option<usize> {
    //let raw_input = fs::read_to_string(p).ok()?;
    //let mut parts = raw_input.split("\n\n");

    //let dict = parts.next().unwrap();
    //let mut grid = parts.next().unwrap().split("\n")
        //.filter(|x| !x.is_empty())
        //.enumerate()
        //.flat_map(|(y, row)| {
            //(0..row.len())
                //.map(move |i| {
                    //match &row[i..=i] {
                        //"#" => ((y as i64, i as i64), true),
                        //_ => ((y as i64, i as i64), false),
                    //}
                //})
        //})
        //.collect::<HashMap<(i64, i64), bool>>();

    //let steps = 50;
    //for i in 0..steps {
        //grid = step_grid(&grid, dict, i)?;
    //}

    //Some(grid.values()
        //.filter(|&x| *x)
        //.count())
//}


fn step_grid(grid: &HashMap<(i64, i64), bool>, dict: &str, step: usize) -> Option<HashMap<(i64, i64), bool>> {
    let infinite_points = &dict[(step % 2)..=(step % 2)] == "#";
    // This is stupid inefficient, but it works. Incoming hot fix later
    let mut next_grid = grid.clone();
    for (y, x) in grid.keys() {
        for y2 in -1..=1 {
            for x2 in -1..=1 {
                let to_check = &(y + y2, x + x2);
                if !next_grid.contains_key(to_check) {
                    next_grid.insert(*to_check, false);
                } 
            }
        }
    }

    for ((y, x), g) in next_grid.iter_mut() {
        let mut pos: usize = 0;
        for y2 in -1..=1 {
            for x2 in -1..=1 {
                pos <<= 1;
                let to_check = &(y + y2, x + x2);
                if grid.contains_key(to_check) && *grid.get(to_check)? {
                    pos |= 1;
                } else if !grid.contains_key(to_check) && !infinite_points {
                    pos |= 1;
                }
            }
        }
        *g = &dict[pos..=pos] == "#";
    }

    Some(next_grid)
}

    //Some(grid.values()
        //.filter(|&x| *x)
        //.count())
    //}

//fn p02(p: &Path) -> Option<i64> {
//let raw_input = fs::read_to_string(p).ok()?;

    //Some(max_manhattan)
//}
