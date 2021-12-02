use std::path::Path;
// crate:: means relevant to this projects root (rust 2018)
use crate::utils::file;

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
    let lines = file::read_to_lines(p);

    let mut count = 0;
    let mut last = i32::MAX;

    for line in lines {
        let int_value = file::line_as_int(line);

        if int_value > last {
            count += 1;
        }

        last = int_value;
    }
    count
}

fn part_2(p: &Path) -> i32 {
    let lines = file::read_to_lines(p);

    let mut stack: Vec<i32> = vec![0];
    let mut count = 0;
    let mut last = i32::MAX;
    let mut next = 0;

    for line in lines {
        let int_value = file::line_as_int(line);

        next += int_value;
        stack.push(int_value);
        if stack.len() < 4 {
            continue;
        }
        next -= stack.remove(0);

        if next > last {
            count += 1;
        }

        last = next;
    }
    count
}
