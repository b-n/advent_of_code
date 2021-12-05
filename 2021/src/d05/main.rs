use crate::types::line;
use crate::utils::file;
use std::collections::HashMap;
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
    Some(
        read_path_to_lines(p)?
            .iter()
            .filter(|l| l.is_straight())
            .flat_map(|l| l.points().unwrap())
            .fold(HashMap::new(), |mut acc, p| {
                *acc.entry(p).or_insert(0) += 1;
                acc
            })
            .values()
            .filter(|&v| *v >= 2)
            .collect::<Vec<&i32>>()
            .len(),
    )
}

fn p02(p: &Path) -> Option<usize> {
    Some(
        read_path_to_lines(p)?
            .iter()
            .flat_map(|l| l.points().unwrap())
            .fold(HashMap::new(), |mut acc, p| {
                *acc.entry(p).or_insert(0) += 1;
                acc
            })
            .values()
            .filter(|&v| *v >= 2)
            .collect::<Vec<&i32>>()
            .len(),
    )
}

fn read_path_to_lines(p: &Path) -> Option<Vec<line::Line>> {
    Some(
        file::read_to_lines(p)
            .map(|l| file::line_as_str(l))
            .map(|l| l.parse::<line::Line>().unwrap())
            .collect::<Vec<line::Line>>(),
    )
}
