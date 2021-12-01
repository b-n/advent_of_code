use std::fs::File;
// What does prelude really do?
use std::io::{prelude::*, self, BufReader};
use std::path::Path;

// Learnings:
// - Many things return a Result<T, E>
//   - .unwrap() to just get the value, .expect("some exception") to throw a custom exception
// - Iterators should just be copied/passed apparently. Passing by ref isn't liked (e.g. &line)
// - Okay, I could have just read all the lines into memory, but it feels "natural" to be using
//   iters - things most languages hide away and/or people ignore
// - My enjoyment of rust so far: [++--------]

pub fn run() {
    let path = Path::new("./input/01");
    println!("{:?}", path);

    println!("Part 1: {}", part_1(path));
    println!("Part 2: {}", part_2(path));
}

fn part_1(p: &Path) -> i32 {
    let lines = read_to_lines(p);

    let mut count = 0;
    let mut last = i32::MAX;

    for line in lines {
        let int_value = line_as_int(line);

        if int_value > last {
            count += 1;
        }

        last = int_value;
    }
    count
}

fn part_2(p: &Path) -> i32 {
    let lines = read_to_lines(p);

    let mut stack: Vec<i32> = vec![0];
    let mut count = 0;
    let mut last = i32::MAX;

    for line in lines {
        let int_value = line_as_int(line);

        stack.push(int_value);
        if stack.len() < 4 {
            continue;
        }
        stack.remove(0);

        let sum: i32 = stack.iter().sum();
        if sum > last {
            count += 1;
        }

        last = sum;
    }
    count
}

fn line_as_int(line: io::Result<String>) -> i32 {
    line_as_str(line).parse::<i32>().unwrap()
}

fn line_as_str(line: io::Result<String>) -> String {
    line.expect("not a line").to_string()
} 

fn read_to_lines(p: &Path) -> io::Lines<BufReader<File>> {
    let file = File::open(p).expect("Could not find file");

    // if the last line doesn't have a semi, it's inferred return
    BufReader::new(file).lines()
}
