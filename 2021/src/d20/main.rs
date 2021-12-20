use std::fs;
use std::path::Path;
use std::collections::HashMap;

pub fn run() {
    let path = Path::new("./input/20");

    println!("Part 1: {}", p01(path).unwrap());
    //println!("Part 2: {}", p02(path).unwrap());
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


    print_grid(&grid);
    println!("{}", grid.values().filter(|&x| *x).count());

    for _ in 0..2 {
        // get all points to eval. current grid + edges
        let mut next_grid = grid.clone();
        for (y, x) in grid.keys() {
             for y2 in -1..=1 {
                for x2 in -1..=1 {
                    let to_check = &(y + y2, x + x2);
                    if !grid.contains_key(to_check) {
                        next_grid.insert(*to_check, false);
                    } 
                }
            }
        }

        for ((y, x), g) in next_grid.iter_mut() {
            let mut pos = 0;
            for y2 in -1..=1 {
                for x2 in -1..=1 {
                    pos <<= 1;
                    let to_check = &(y + y2, x + x2);
                    if grid.contains_key(to_check) && *grid.get(to_check)? {
                        pos |= 1;
                    }
                }
            }
            *g = &dict[pos..=pos] == "#";
        }

        grid = next_grid;
        print_grid(&grid);
        println!("{}", grid.values().filter(|&x| *x).count());

    }
    
    Some(grid.values()
        .filter(|&x| *x)
        .count())
}

//fn p02(p: &Path) -> Option<i64> {
    //let raw_input = fs::read_to_string(p).ok()?;

    //Some(max_manhattan)
//}

fn print_grid(chart: &HashMap<(i64, i64), bool>) -> Option<()> {
    for y in -3..103 {
        for x in -3..103 {
            if chart.contains_key(&(y, x)) && *chart.get(&(y, x))? {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    Some(())
}
