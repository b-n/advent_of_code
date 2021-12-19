use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Beacon {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl FromStr for Beacon {
    type Err = std::num::ParseIntError;
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut parts = input.split(",").map(|x| x.parse::<i64>().unwrap());

        Ok(Beacon {
            x: parts.next().unwrap(),
            y: parts.next().unwrap(),
            z: parts.next().unwrap(),
        })
    }
}

impl std::ops::Sub for Beacon {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
