use std::str::FromStr;

#[derive(Debug)]
pub enum Direction {
    Forward,
    Down,
    Up
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "up"      => Ok(Self::Up),
            "down"    => Ok(Self::Down),
            _         => Err(()),
        }
    }
}
