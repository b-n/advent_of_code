use std::path::Path;
use crate::utils::file;

//Learnings:
// - going straight to iteration now, instead of first writing loops. This feels better
// - I should abstract some of my solution more, but naive woo yay
// - It's kind of annoying getting refs to things (e.g. min/max gives a ref), but i understand why
// - I may come back and revisit a split search since that'd be way more effective
// - TRIANGLE NUMBERS are easy to calculate!
// Rust Enjoyment factor: [++++------] +1/2 ?

pub fn run() {
    let path = Path::new("./input/07");

    println!("Part 1: {}", p01(path).unwrap());
    println!("Part 2: {}", p02(path).unwrap());
}

fn p01(p: &Path) -> Option<i64> {
    let raw_input = file::line_as_str(file::read_to_lines(p).next()?)?;
    
    let crabs = csv_to_vec(raw_input)?;

    // This is computationally inefficient. It would be quicker to do a tree search
    let costs = (0..1000)
        .map(|i| {
            crabs.iter()
                .map(|c| (c - i).abs())
                .sum()
        }).collect::<Vec<i64>>();

    Some(*costs.iter().min()?)
}

fn p02(p: &Path) -> Option<i64> {
    let raw_input = file::line_as_str(file::read_to_lines(p).next()?)?;

    let crabs = csv_to_vec(raw_input)?;

    // As P1, could be a bit faster here
    let costs = (0..1000)
        .map(|i| {
            crabs.iter()
                .map(|c| (c - i).abs())
                .map(|c| c * (c + 1) /2)
                .sum()
        }).collect::<Vec<i64>>();

    Some(*costs.iter().min()?)
}

fn csv_to_vec(s: String) -> Option<Vec<i64>> {
    Some(s.split(",").map(|x| x.parse::<i64>().unwrap()).collect())
}
