use crate::utils::{file, math, vec};
use std::path::Path;

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

fn p01(p: &Path) -> Option<usize> {
    let raw_input = file::line_as_str(file::read_to_lines(p).next()?)?;

    let crabs = file::csv_to_vec::<usize>(raw_input).ok()?;

    let vec::MinMax { min, max } = vec::min_max(&crabs);

    // This is computationally inefficient. It would be quicker to do a tree search
    let costs = (min..=max)
        .map(|i| crabs.iter().map(|c| math::abs_diff(c, &i)).sum())
        .collect::<Vec<usize>>();

    Some(*costs.iter().min()?)
}

fn p02(p: &Path) -> Option<usize> {
    let raw_input = file::line_as_str(file::read_to_lines(p).next()?)?;

    let crabs = file::csv_to_vec::<usize>(raw_input).ok()?;

    let vec::MinMax { min, max } = vec::min_max(&crabs);

    // As P1, could be a bit faster here
    let costs = (min..=max)
        .map(|i| {
            crabs
                .iter()
                .map(|c| math::abs_diff(c, &i))
                .map(|c| c * (c + 1) / 2)
                .sum()
        })
        .collect::<Vec<usize>>();

    Some(*costs.iter().min()?)
}
