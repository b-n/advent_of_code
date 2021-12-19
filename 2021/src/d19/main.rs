use std::fs;
use std::path::Path;
use super::{scanner::Scanner};

pub fn run() {
    let path = Path::new("./input/19_basic");

    println!("Part 1: {}", p01(path).unwrap());
    //println!("Part 2: {}", p02(path).unwrap());
}

fn p01(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;

    let scanners = raw_input.split("\n\n")
        .map(|x| x.parse::<Scanner>().unwrap())
        .collect::<Vec<Scanner>>();

    println!("{:?}", scanners);
    Some(0)
}

//fn p02(p: &Path) -> Option<usize> {
    //let raw_input = fs::read_to_string(p).ok()?;

    //Some(0)
//}
