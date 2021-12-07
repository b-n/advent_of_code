use std::fmt::Debug;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
// prelude is actually a module which is needed by BufReader funcs etc
//   ref: https://doc.rust-lang.org/stable/std/io/prelude/index.html
use std::io;
use std::io::{prelude::*, BufReader, Lines};

#[allow(dead_code)]
pub fn line_as_usize(line: io::Result<String>) -> Option<usize> {
    Some(line_as_str(line)?.parse::<usize>().ok()?)
}

#[allow(dead_code)]
pub fn line_as_str(line: io::Result<String>) -> Option<String> {
    Some(line.ok()?.to_string())
}

#[allow(dead_code)]
pub fn lines_as_vec2d(lines: Lines<BufReader<File>>) -> Option<Vec<Vec<char>>> {
    Some(
        lines
            .map(|l| line_as_str(l))
            .map(|l| l.map(|line| line.chars().collect::<Vec<char>>()))
            .flat_map(|x| x)
            .collect::<Vec<Vec<char>>>(),
    )
}

#[allow(dead_code)]
pub fn csv_to_vec<T>(s: String) -> Result<Vec<T>, T::Err>
where
    T: FromStr,
    <T as FromStr>::Err: Debug, // The T that looks like FromStr has an Err which implements Debug
{
    s.split(",")
        .map(|x| x.parse::<T>())
        .collect()
}

#[allow(dead_code)]
pub fn read_to_lines(p: &Path) -> Lines<BufReader<File>> {
    let file = File::open(p).expect("Could not find file");

    BufReader::new(file).lines()
}
