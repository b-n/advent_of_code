use crate::types::line;
use crate::types::point;
use crate::utils::file;
use std::path::Path;

//Learnings
// - if you're implementing FromStr, you should use parse, not the function itself?? (why?)
// - Early optimization makes code look bad

type Extent = (usize, usize);

pub fn run() {
    let path = Path::new("./input/05");

    println!("Part 1: {}", p01(&path).unwrap());
    //println!("Part 2: {}", p02(&path).unwrap());
}

fn p01(p: &Path) -> Option<usize> {
    let lines = read_path_to_lines(p)?;

    let straight_lines = lines
        .iter()
        .filter(|l| l.is_straight())
        .collect::<Vec<&line::Line>>();

    let extents = line_extents(&straight_lines);

    let mut dangerous_points = 0;
    //let mut output: Vec<Vec<usize>> = vec![vec![0; extents.0 .1 - extents.0 .0 + 1]; extents.1 .1 - extents.1 .0 + 1];

    for y in extents.1 .0..=extents.1 .1 {
        for x in extents.0 .0..=extents.0 .1 {
            let p = point::Point { x, y };
            let mut intersecting_lines = 0;
            for line in straight_lines.iter() {
                if line.is_point_in_line(&p) {
                    intersecting_lines += 1;
                }
                if intersecting_lines >= 2 {
                    break;
                }
            }

            //output[y - extents.1 .0][x - extents.0 .0] = intersecting_lines;
            if intersecting_lines >= 2 {
                dangerous_points += 1;
            }
        }
    }

    //for line in output.iter() { println!("{:?}", line); }

    Some(dangerous_points)
}

// I should really implement something better here, I do this elsewhere too
// This really is also a tiny optimization
fn line_extents(lines: &Vec<&line::Line>) -> (Extent, Extent) {
    lines
        .iter()
        .fold(((usize::MAX, 0), (usize::MAX, 0)), |mut acc, v| {
            if v.start.x < acc.0 .0 {
                acc.0 .0 = v.start.x
            }
            if v.end.x < acc.0 .0 {
                acc.0 .0 = v.end.x
            }
            if v.start.y < acc.1 .0 {
                acc.1 .0 = v.start.y
            }
            if v.end.y < acc.1 .0 {
                acc.1 .0 = v.end.y
            }

            if v.start.x > acc.0 .1 {
                acc.0 .1 = v.start.x
            }
            if v.end.x > acc.0 .1 {
                acc.0 .1 = v.end.x
            }
            if v.start.y > acc.1 .1 {
                acc.1 .1 = v.start.y
            }
            if v.end.y > acc.1 .1 {
                acc.1 .1 = v.end.y
            }
            acc
        })
}

fn read_path_to_lines(p: &Path) -> Option<Vec<line::Line>> {
    Some(
        file::read_to_lines(p)
            .map(|l| file::line_as_str(l))
            .map(|l| l.parse::<line::Line>().unwrap())
            .collect::<Vec<line::Line>>(),
    )
}
