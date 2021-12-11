use crate::types::{chart::Chart, point3d::Point3d};
use crate::utils::file;
use std::path::Path;

pub fn run() {
    let path = Path::new("./input/11");

    println!("Part 1: {}", p01(path).unwrap());
    println!("Part 2: {}", p02(path).unwrap());
}

fn p01(p: &Path) -> Option<usize> {
    let lines = file::read_to_lines(p);

    let mut chart = Chart::from_2d_vec(&file::lines_as_vec2d_usize(lines)?);

    //println!("{}", chart);
    let dirs: Vec<(i32, i32)> = vec![
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    let steps = 100;

    let mut total_flashes: usize = 0;
    for _ in 0..steps {
        chart.inc(1);

        let mut flashed_points: Vec<Point3d> = vec![];
        while let Some(_) = cycle_step(&mut chart, &mut flashed_points, &dirs) {}

        total_flashes += chart
            .iter_mut()
            .map(|p| {
                if p.z >= 10 {
                    p.z = 0;
                    1
                } else {
                    0
                }
            })
            .sum::<usize>();

        //println!("{}", chart);
    }
    Some(total_flashes)
}

fn p02(p: &Path) -> Option<usize> {
    let lines = file::read_to_lines(p);

    let mut chart = Chart::from_2d_vec(&file::lines_as_vec2d_usize(lines)?);

    //println!("{}", chart);
    let dirs: Vec<(i32, i32)> = vec![
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    let mut step: usize = 1; //we start at step 1....
    loop {
        chart.inc(1);

        let mut flashed_points: Vec<Point3d> = vec![];
        while let Some(_) = cycle_step(&mut chart, &mut flashed_points, &dirs) {}

        chart
            .iter_mut()
            .map(|p| {
                if p.z >= 10 {
                    p.z = 0;
                    1
                } else {
                    0
                }
            })
            .sum::<usize>();
        
        //println!("{}", chart);
        
        // if min = max, I guess we're all the same!
        let values = chart.iter().map(|p| p.z).collect::<Vec<usize>>();
        if values.iter().min() == values.iter().max() {
            break;
        }

        step += 1;
    }

    Some(step)
}

fn cycle_step(
    chart: &mut Chart,
    flashed_points: &mut Vec<Point3d>,
    dirs: &Vec<(i32, i32)>,
) -> Option<usize> {
    let to_flash: Vec<Point3d> = chart
        .iter()
        .filter(|p| !flashed_points.contains(p))
        .filter(|p| p.z >= 10)
        .map(|p| *p)
        .collect::<Vec<Point3d>>();

    let flashed: usize = to_flash
        .iter()
        .map(|p| flash_point(chart, p, dirs))
        .map(|p| flashed_points.push(*p))
        .count();

    match flashed {
        p if p > 0 => Some(p),
        _ => None,
    }
}

// Okay i finally figured out what 'a means. It's saying where it's borrowed from. fun
fn flash_point<'a>(chart: &mut Chart, p: &'a Point3d, dirs: &Vec<(i32, i32)>) -> &'a Point3d {
    for (x, y) in dirs.iter() {
        let p2 = chart.at_pos((p.x as i32 + x) as usize, (p.y as i32 + y) as usize);
        match p2 {
            Some(p2) => p2.z += 1,
            None => (),
        }
    }
    // returning the same input value so we can just chain it in an iterator (i'm sure there's
    // better syntax for this
    p
}
