use crate::types::line;
use crate::utils::file;
use std::path::Path;

//Learnings
// - if you're implementing FromStr, you should use parse, not the function itself?? (why?)
// - Early optimization makes code look bad
// - Algorithms are better when you think them through (e.g. where can a point be)
// - Yeah iterators are nice
// - My previous rust code was very C in rust. I think I'm gettting the hang of this rust thing a
//   little bit
// 
// Rust enjoyment factor [+++/------] (no change)

pub fn run() {
    let path = Path::new("./input/05");

    println!("Part 1: {}", p01(&path).unwrap());
    println!("Part 2: {}", p02(&path).unwrap());
}

fn p01(p: &Path) -> Option<usize> {
    let raw_lines = read_path_to_lines(p)?;

    let lines = raw_lines
        .iter()
        .filter(|l| l.is_straight())
        .collect::<Vec<&line::Line>>();

    let (max_x, max_y) = lines
        .iter()
        .map(|l| {
            (
                std::cmp::max(l.start.x, l.end.x),
                std::cmp::max(l.start.y, l.end.y),
            )
        })
        .reduce(|acc, item| (std::cmp::max(acc.0, item.0), std::cmp::max(acc.1, item.1)))?;

    let mut output: Vec<Vec<usize>> = vec![vec![0; max_x + 1]; max_y + 1];

    for line in lines {
        for point in line.points()? {
            output[point.y][point.x] += 1;
        }
    }

    let mut intersections = 0;
    for row in output {
        for val in row {
            if val >= 2 {
                intersections += 1
            }
        }
    }

    Some(intersections)
}

fn p02(p: &Path) -> Option<usize> {
    let lines = read_path_to_lines(p)?;

    let mut output: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];

    for line in lines {
        for point in line.points()? {
            //println!("{:?}", point);
            output[point.y][point.x] += 1;
        }
    }

    //for line in output.iter() { println!("{:?}", line); }

    let mut intersections = 0;
    for row in output {
        for val in row {
            if val >= 2 {
                intersections += 1
            }
        }
    }

    Some(intersections)
}

fn read_path_to_lines(p: &Path) -> Option<Vec<line::Line>> {
    Some(
        file::read_to_lines(p)
            .map(|l| file::line_as_str(l))
            .map(|l| l.parse::<line::Line>().unwrap())
            .collect::<Vec<line::Line>>(),
    )
}
