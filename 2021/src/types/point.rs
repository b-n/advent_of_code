use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(",")
            .collect::<Vec<&str>>();

        let x = parts[0].parse::<usize>()?;
        let y = parts[1].parse::<usize>()?;

        Ok(Point { x, y })
    }
}
