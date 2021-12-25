use std::fs;
use std::path::Path;

pub fn run() {
    let path = Path::new("./input/25");

    println!("Part 1: {}", p01(&path).unwrap());
}

fn w(i: usize, max: usize) -> usize {
    if i >= max {
        0
    } else {
        i
    }
}

fn p01(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;

    let grid = raw_input.split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let (max_y, max_x) = (grid.len(), grid[0].len());
    let mut last = vec![];
    let mut current = grid;
    
    let mut grids = 0;
    while last != current {
        last = current.clone();
        let mut next = current.clone();
        for (y, row) in current.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                if *v == '>' {
                    if current[y][w(x + 1, max_x)] == '.' {
                        next[y][x] = '.';
                        next[y][w(x + 1, max_x)] = '>';
                    }
                }
            }
        }
        current = next.clone();
        for (y, row) in current.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                if *v == 'v' {
                    if current[w(y + 1, max_y)][x] == '.' {
                        next[y][x] = '.';
                        next[w(y + 1, max_y)][x] = 'v';
                    }
                }
            }
        }
        current = next;
        grids += 1;
    }

    Some(grids)
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for v in row {
            print!("{}", v);
        }
        println!();
    }
    println!();
}
