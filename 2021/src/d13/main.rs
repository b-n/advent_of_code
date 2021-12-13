use crate::types::point::Point;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn run() {
    let path = Path::new("./input/13");

    println!("Part 1: {}", p01(path).unwrap());
    println!("Part 2: {}", p02(path).unwrap());
}

fn p01(p: &Path) -> Option<usize> {
    let (initial_points, folds) = parse_input(p)?;
    let fold = folds[0];
    let result = fold_points(&initial_points, fold.0, fold.1)?;
    Some(result.len())
}

fn p02(p: &Path) -> Option<usize> {
    let (initial_points, folds) = parse_input(p)?;

    let mut result = initial_points;
    for fold in folds {
        result = fold_points(&result, fold.0, fold.1)?;
    }
    print_points(&result);
    Some(0)
}

#[allow(dead_code)]
fn print_points(input: &HashSet<Point>) -> Option<()> {
    let y_max = input.iter().map(|p| p.y).max()?;
    let x_max = input.iter().map(|p| p.x).max()?;
    for y in 0..=y_max {
        for x in 0..=x_max {
            if input.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    Some(())
}

fn fold_points(input: &HashSet<Point>, axis: char, pos: usize) -> Option<HashSet<Point>> {
    match axis {
        'x' => Some(
            input
                .iter()
                .map(|&p| {
                    if p.x < pos {
                        p
                    } else {
                        let new_x = (pos -1) - (p.x - (pos + 1));
                        Point {
                            x: new_x,
                            y: p.y,
                        }
                    }
                })
                .collect()
        ),
        'y' => Some(
            input
                .iter()
                .map(|&p| {
                    if p.y < pos {
                        p
                    } else {
                        let new_y = (pos - 1) - (p.y - (pos + 1));
                        Point {
                            x: p.x,
                            y: new_y,
                        }
                    }
                })
                .collect(),
        ),
        _ => None,
    }
}

fn parse_input(p: &Path) -> Option<(HashSet<Point>, Vec<(char, usize)>)> {
    let mut file = File::open(p).ok()?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).ok()?;

    let parts = buffer.split("\n\n").collect::<Vec<&str>>();

    let points = parts[0]
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse::<Point>().unwrap())
        .collect();

    let folds = parts[1]
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|&s| s != &"")
        .map(|s| {
            let fold_parts = s.split("=").collect::<Vec<&str>>();
            let axis = fold_parts[0].chars().nth(11).unwrap();
            let pos = fold_parts[1].parse::<usize>().unwrap();
            (axis, pos)
        })
        .collect::<Vec<(char, usize)>>();

    Some((points, folds))
}
