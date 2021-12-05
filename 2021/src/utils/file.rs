use std::fs::File;
use std::path::Path;
// prelude is actually a module which is needed by BufReader funcs etc
//   ref: https://doc.rust-lang.org/stable/std/io/prelude/index.html
use std::io::{prelude::*, BufReader, Lines, Result};

#[allow(dead_code)]
pub fn line_as_usize(line: Result<String>) -> Option<usize> {
    Some(line_as_str(line)?.parse::<usize>().ok()?)
}

#[allow(dead_code)]
pub fn line_as_str(line: Result<String>) -> Option<String> {
    Some(line.ok()?.to_string())
}

#[allow(dead_code)]
pub fn lines_as_vec2d(lines: Lines<BufReader<File>>) -> Option<Vec<Vec<char>>> {
    Some(
        lines
            .map(|l| line_as_str(l))
            .map(|l| l.map(|line| line.chars().collect::<Vec<char>>()))
            .flat_map(|x| x)
            .collect::<Vec<Vec<char>>>()
        )
}

#[allow(dead_code)]
pub fn read_to_lines(p: &Path) -> Lines<BufReader<File>> {
    let file = File::open(p).expect("Could not find file");

    BufReader::new(file).lines()
}
