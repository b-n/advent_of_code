use crate::types::{chart::Chart, point3d::Point3d};
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

    let mut total_flashes: usize = 0;
    for _ in 0..steps {
        chart.inc(1);

        let mut flashed_points: Vec<Point3d> = vec![];
        while let Some(_) = cycle_step(&mut chart, &mut flashed_points) {}

        total_flashes += chart
            .iter_mut()
            .filter(|p| p.z >= 10)
            .map(|p| p.z = 0)
            .count();
    }
    Some(total_flashes)
}

fn p02(p: &Path) -> Option<usize> {
    let lines = file::read_to_lines(p);

    let mut chart = Chart::from_2d_vec(&file::lines_as_vec2d_usize(lines)?);

    let mut step: usize = 1; //we start at step 1....
    loop {
        chart.inc(1);

        let mut flashed_points: Vec<Point3d> = vec![];
        while let Some(_) = cycle_step(&mut chart, &mut flashed_points) {}

        for p in chart.iter_mut() {
            if p.z >= 10 {
                p.z = 0
            }
        }

        // if min = max, I guess we're all the same!
        let values = chart.iter().map(|p| p.z).collect::<Vec<usize>>();
        if values.iter().min() == values.iter().max() {
            break;
        }

        step += 1;
    }

    Some(step)
}

fn cycle_step(chart: &mut Chart, flashed_points: &mut Vec<Point3d>) -> Option<usize> {
    let flashed: usize = chart
        .iter()
        .filter(|p| !flashed_points.contains(p) && p.z >= 10)
        .map(|p| *p)
        .collect::<Vec<Point3d>>() // stops borrow on chart and flashed_points
        .iter()
        .map(|p| {
            flashed_points.push(*p);
            flash_point(chart, *p);
        })
        .count();

    match flashed {
        p if p > 0 => Some(p),
        _ => None,
    }
}

fn flash_point(chart: &mut Chart, p: Point3d) {
    for (x, y) in DIRS.iter() {
        let p2 = chart.at_pos((p.x as i32 + x) as usize, (p.y as i32 + y) as usize);
        match p2 {
            Some(p2) => p2.z += 1,
            None => (),
        }
    }
}
