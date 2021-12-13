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

#[derive(Clone, Copy)]
#[repr(u8)]
enum Axis {
    Y = b'y',
    X = b'x',
}

fn p01(p: &Path) -> Option<usize> {
    let (initial_points, folds) = parse_input(p)?;
    let fold = folds[0];
    let result = fold_points(&initial_points, fold.0, fold.1)?;
    Some(result.len())
}

fn p02(p: &Path) -> Option<usize> {
    let (initial_points, folds) = parse_input(p)?;

    let result = folds.iter()
        .fold(initial_points, |acc, fold| {
            fold_points(&acc, fold.0, fold.1).unwrap()
        });
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

fn fold_points(input: &HashSet<Point>, axis: Axis, pos: usize) -> Option<HashSet<Point>> {
    let attr_reader = |p: Point| -> usize {
        match axis {
            Axis::Y => p.y,
            Axis::X => p.x,
        }
    };

    let attr_writer = |p: Point| -> (usize, usize) {
        match axis {
            Axis::Y => (p.x, 2 * pos - p.y),
            Axis::X => (2 * pos - p.x, p.y),
        }
    };

    Some(
        input
            .iter()
            .map(|&p| {
                if attr_reader(p) < pos {
                    p
                } else {
                    let (x, y) = attr_writer(p);
                    Point { x, y }
                }
            })
            .collect(),
    )
}

fn parse_input(p: &Path) -> Option<(HashSet<Point>, Vec<(Axis, usize)>)> {
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
            let axis = match fold_parts[0].chars().nth(11).unwrap() {
                'x' => Axis::X,
                _ => Axis::Y,
            };
            let pos = fold_parts[1].parse::<usize>().unwrap();
            (axis, pos)
        })
        .collect::<Vec<(Axis, usize)>>();

    Some((points, folds))
}
