use std::path::Path;
use crate::utils::*;

// Learnings:
// - I should probably learn to parse Strings to Enums
// - String::from feels a little bit like an antipattern in the grand scheme
// - I really should use a struct for the Vehicle and use functions on it (more fun)
//
// My enjoyment of rust so far: [+++-------]

pub fn run() {
    let path = Path::new("./input/02");

    println!("Part 1: {}", p01(&path));
    println!("Part 2: {}", p02(&path));
}

fn p01(p: &Path) -> i32 {
    let lines = utils::read_to_lines(p);

    let mut depth = 0;
    let mut x = 0;

    for line in lines {
        let str_value = utils::line_as_str(line);

        let (direction, distance) = str_to_command(&str_value);

        match direction {
            "forward" => x += distance,
            "down" => depth += distance,
            "up" => depth -= distance,
            _ => println!("Bad command"),
        }
    }
    
    depth * x
}

fn p02(p: &Path) -> i32 {
    let lines = utils::read_to_lines(p);

    let mut depth = 0;
    let mut x = 0;
    let mut aim = 0;

    for line in lines {
        let str_value = utils::line_as_str(line);

        let (direction, distance) = str_to_command(&str_value);

        match direction {
            "forward" => {
                x += distance;
                depth += aim * distance;
            },
            "down" => aim  += distance,
            "up" => aim -= distance,
            _ => println!("Bad command"),
        }
    }
    
    depth * x
}

// This needed to be a ref to the String, not the String itself. Why?
// str_to_command(some_var) would actually generate a new string, and then rust says ownership is
// in this function. That means the returning value of `direction` would fail, because it's value
// and ownership (i guess?) gets destroyed after the function stops. So, passing a ref in here
// works, because the function therefore doesn't "own" anything (everything is derrived), and that
// means it can return something based off the original creation. :mind_blown: (I hope this is
// fact)
fn str_to_command(input: &String) -> (&str, i32) {
    let mut raw_command = input.split(" ");
    // Nice to use iterators, but... coercion should be easier than a bunch of unwrap()s everywhere
    let direction = raw_command.next().unwrap();
    let distance = raw_command.next().unwrap().parse::<i32>().unwrap();

    (direction, distance)
}

