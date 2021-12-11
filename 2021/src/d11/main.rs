use super::{chart::Chart, point3d::Point3d};
use crate::utils::file;
use std::path::Path;

pub fn run() {
    let path = Path::new("./input/11");

    println!("Part 1: {}", p01(path).unwrap());
    println!("Part 2: {}", p02(path).unwrap());
}
const DIRS: &'static [(i32, i32)] = &[
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn p01(p: &Path) -> Option<usize> {
    let lines = file::read_to_lines(p);
    let mut chart = Chart::from_2d_vec(&file::lines_as_vec2d_usize(lines)?);

    let steps = 100;
    Some(
        (0..steps)
            .map(|_| {
                chart.inc(1);

                let mut flashed_points: Vec<Point3d> = vec![];
                while let Some(_) = cycle_step(&mut chart, &mut flashed_points) {}

                chart.reset(|p| p.z >= 10)
            })
            .sum(),
    )
}

fn p02(p: &Path) -> Option<usize> {
    let lines = file::read_to_lines(p);
    let mut chart = Chart::from_2d_vec(&file::lines_as_vec2d_usize(lines)?);

    let mut step: usize = 1; //we start at step 1....
    'next: loop {
        chart.inc(1);

        let mut flashed_points: Vec<Point3d> = vec![];
        while let Some(_) = cycle_step(&mut chart, &mut flashed_points) {}

        chart.reset(|p| p.z >= 10);
        
        //uniformity only happens on 0 since it takes a flash to get there
        for p in chart.iter() {
            if p.z != 0 {
                step += 1;
                continue 'next;
            }
        }
        break;
    }

    Some(step)
}

// The code looks spaghet, but it's interesting why
// Since the first filter/map is operating on a borrowed reference from chart/flashed_points, then
// it's prevents any further get/mutations. This is why we collect in the middle, since it removes
// any borrows (all refs are flattened into values)
fn cycle_step(chart: &mut Chart, flashed_points: &mut Vec<Point3d>) -> Option<usize> {
    match chart
        .iter()
        .filter(|p| !flashed_points.contains(p) && p.z >= 10)
        .map(|p| *p)
        .collect::<Vec<Point3d>>() // stops borrow on chart and flashed_points
        .iter()
        .map(|p| {
            flashed_points.push(*p);
            flash_point(chart, *p);
        })
        .count()
    {
        p if p > 0 => Some(p),
        _ => None,
    }
}

// this is kind of cool with Option:
// - chart.at_pos returns an option.
//   - If it's None, then it's out of bounds (e.g. couldn't find the point)
//   - otherwise it returns the mutateable point
fn flash_point(chart: &mut Chart, p: Point3d) {
    for (x, y) in DIRS.iter() {
        match chart.at_pos((p.x as i32 + x) as usize, (p.y as i32 + y) as usize) {
            Some(p2) => p2.z += 1,
            None => (),
        }
    }
}
