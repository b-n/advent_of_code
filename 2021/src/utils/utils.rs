use std::fs::File;
use std::path::Path;
// prelude is actually a module which is needed by BufReader funcs etc
//   ref: https://doc.rust-lang.org/stable/std/io/prelude/index.html
use std::io::{prelude::*, Result, Lines, BufReader};

pub fn line_as_int(line: Result<String>) -> i32 {
    line_as_str(line).parse::<i32>().unwrap()
}

pub fn line_as_str(line: Result<String>) -> String {
    line.expect("not a line").to_string()
} 

pub fn read_to_lines(p: &Path) -> Lines<BufReader<File>> {
    let file = File::open(p).expect("Could not find file");

    // if the last line doesn't have a semi, it's inferred return
    BufReader::new(file).lines()
}
