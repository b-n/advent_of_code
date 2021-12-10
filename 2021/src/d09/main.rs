use crate::types::point::Point;
use crate::utils::file;
use std::collections::HashSet;
use std::io::{BufReader, Lines};
use std::path::Path;

pub fn run() {
    let path = Path::new("./input/09");

    println!("Part 1: {}", p01(path).unwrap());
    println!("Part 2: {}", p02(path).unwrap());
}

fn p01(p: &Path) -> Option<usize> {
    let lines = file::read_to_lines(p);

    let chart = lines_to_i32(lines)?;

    let mut low_values: Vec<usize> = vec![];

    let dirs: Vec<(i32, i32)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    for (y, l) in chart.iter().enumerate() {
        for (x, z) in l.iter().enumerate() {
            let adjacent_min = lowest_surrounding(&chart, x as i32, y as i32, &dirs)?;

            if adjacent_min > *z {
                low_values.push(*z as usize);
            }
        }
    }

    //print_chart(&chart);

    Some(low_values.iter().fold(0, |acc, x| acc + x + 1))
}

fn p02(p: &Path) -> Option<usize> {
    let lines = file::read_to_lines(p);

    let chart = lines_to_i32(lines)?;

    let mut low_points: Vec<Point> = vec![];

    let dirs: Vec<(i32, i32)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    for (y, l) in chart.iter().enumerate() {
        for (x, z) in l.iter().enumerate() {
            let adjacent_min = lowest_surrounding(&chart, x as i32, y as i32, &dirs)?;

            if adjacent_min > *z {
                low_points.push(Point { x, y });
            }
        }
    }

    let mut basin_sizes = low_points
        .iter()
        .map(|lp| {
            let mut visited_points: HashSet<Point> = HashSet::new();

            get_basin_size(&chart, lp, &dirs, &mut visited_points).unwrap()
        })
        .collect::<Vec<usize>>();

    basin_sizes.sort_by(|a, b| b.cmp(a));

    Some(basin_sizes[0] * basin_sizes[1] * basin_sizes[2])
}

fn get_basin_size(
    chart: &Vec<Vec<i32>>,
    start: &Point,
    dirs: &Vec<(i32, i32)>,
    visited: &mut HashSet<Point>,
) -> Option<usize> {
    let mut size = 1;

    visited.insert(*start);
    for (x, y) in dirs.iter() {
        let adj_x: usize = (start.x as i32 + *x) as usize;
        let adj_y: usize = (start.y as i32 + *y) as usize;
        let p = Point { x: adj_x, y: adj_y };

        // if visited already, skip
        if visited.contains(&p) {
            continue;
        }
        let adj_val = value_in_chart(chart, adj_x, adj_y);

        // if value of adj < 9, then we recurse
        if adj_val < 9 {
            size += get_basin_size(chart, &p, dirs, visited)?;
        }
    }

    Some(size)
}

fn value_in_chart(chart: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    // Hax, usize is bound, so -1 = usize::MAX - 1 (bigger than the chart size)
    if x >= chart[0].len() || y >= chart.len() {
        9
    } else {
        chart[y][x]
    }
}

fn lowest_surrounding(
    chart: &Vec<Vec<i32>>,
    x: i32,
    y: i32,
    dirs: &Vec<(i32, i32)>,
) -> Option<i32> {
    dirs.iter()
        .map(|(step_x, step_y)| {
            let comp_x = (x + step_x) as usize;
            let comp_y = (y + step_y) as usize;
            value_in_chart(chart, comp_x, comp_y)
        })
        .min()
}

fn lines_to_i32(lines: Lines<BufReader<std::fs::File>>) -> Option<Vec<Vec<i32>>> {
    let char0 = '0'.to_digit(10)?;
    Some(
        lines
            .map(|l| file::line_as_str(l))
            .map(|l| {
                l.map(|line| {
                    line.chars()
                        .map(|c| (c.to_digit(10).unwrap() - char0) as i32)
                        .collect::<Vec<i32>>()
                })
            })
            .flat_map(|x| x)
            .collect::<Vec<Vec<i32>>>(),
    )
}

#[allow(dead_code)]
fn print_chart(c: &Vec<Vec<i32>>) {
    for l in c.iter() {
        println!("{:?}", l);
    }
}
