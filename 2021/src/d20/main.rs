use std::fs;
use std::path::Path;
use std::collections::HashMap;

pub fn run() {
    let path = Path::new("./input/20");

    println!("Part 1: {}", p01(path).unwrap());
    println!("Part 2: {}", p02(path).unwrap());
}

fn p01(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;
    let mut parts = raw_input.split("\n\n");

    let dict = parts.next().unwrap();
    let mut grid = parts.next().unwrap().split("\n")
        .filter(|x| !x.is_empty())
        .enumerate()
        .flat_map(|(y, row)| {
            (0..row.len())
                .map(move |i| {
                    match &row[i..=i] {
                        "#" => ((y as i64, i as i64), true),
                        _ => ((y as i64, i as i64), false),
                    }
                })
        })
        .collect::<HashMap<(i64, i64), bool>>();

    let steps = 2;
    for i in 0..steps {
        grid = step_grid(&grid, dict, i)?;
    }

    Some(grid.values()
        .filter(|&x| *x)
        .count())
}

fn p02(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;
    let mut parts = raw_input.split("\n\n");

    let dict = parts.next().unwrap();
    let mut grid = parts.next().unwrap().split("\n")
        .filter(|x| !x.is_empty())
        .enumerate()
        .flat_map(|(y, row)| {
            (0..row.len())
                .map(move |i| {
                    match &row[i..=i] {
                        "#" => ((y as i64, i as i64), true),
                        _ => ((y as i64, i as i64), false),
                    }
                })
        })
        .collect::<HashMap<(i64, i64), bool>>();

    let steps = 50;
    for i in 0..steps {
        grid = step_grid(&grid, dict, i)?;
    }

    Some(grid.values()
        .filter(|&x| *x)
        .count())
}


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
