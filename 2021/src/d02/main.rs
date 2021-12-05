use crate::types::direction::{Direction, Direction::*};
use crate::utils::file;
use std::path::Path;

// Learnings:
// - I should probably learn to parse Strings to Enums
// - String::from feels a little bit like an antipattern in the grand scheme
// - I really should use a struct for the Vehicle and use functions on it (more fun)
//
// My enjoyment of rust so far: [+++-------]

pub fn run() {
    let path = Path::new("./input/02");

    println!("Part 1: {}", p01(&path).unwrap());
    println!("Part 2: {}", p02(&path).unwrap());
}

fn p01(p: &Path) -> Option<i32> {
    let lines = file::read_to_lines(p);

    let mut depth = 0;
    let mut x = 0;

    for line in lines {
        let str_value = file::line_as_str(line)?;

        let (direction, distance) = str_to_command(&str_value)?;

        match direction {
            Forward => x += distance,
            Down => depth += distance,
            Up => depth -= distance,
        }
    }

    Some(depth * x)
}

fn p02(p: &Path) -> Option<i32> {
    let lines = file::read_to_lines(p);

    let mut depth = 0;
    let mut x = 0;
    let mut aim = 0;

    for line in lines {
        let str_value = file::line_as_str(line)?;

        let (direction, distance) = str_to_command(&str_value)?;

        match direction {
            Forward => {
                x += distance;
                depth += aim * distance;
            }
            Down => aim += distance,
            Up => aim -= distance,
        }
    }

    Some(depth * x)
}

// This needed to be a ref to the String, not the String itself. Why?
// str_to_command(some_var) would actually generate a new string, and then rust says ownership is
// in this function. That means the returning value of `direction` would fail, because it's value
// and ownership (i guess?) gets destroyed after the function stops. So, passing a ref in here
// works, because the function therefore doesn't "own" anything (everything is derrived), and that
// means it can return something based off the original creation. :mind_blown: (I hope this is
// fact)
fn str_to_command(input: &String) -> Option<(Direction, i32)> {
    // We return an Option. Why? I think it's because we're handling errors now
    let mut raw_command = input.split(" ");
    // Since the enum implements FromStr, we're able to &str.parse::<EnumType>() it now.Pretty cool
    let direction = raw_command.next()?.parse::<Direction>().ok()?;
    // How did we get rid of unwrap on the next? (since it's a Result and all), well with the ?.
    // This (from what I understand) changes the Error into a None since we return an Option
    let distance = raw_command.next()?.parse::<i32>().ok()?;

    Some((direction, distance))
}
