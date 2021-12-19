use std::str::FromStr;

#[allow(dead_code)]
pub enum Plane {
    XY,
    YZ,
    XZ,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Beacon {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Beacon {
    pub fn rotate(&mut self, plane: &Plane) {
        match plane {
            Plane::XY => {
                let newx = -1 * self.y;
                self.y = self.x;
                self.x = newx;
            },
            Plane::YZ => {
                let newy = -1 * self.z;
                self.z = self.y;
                self.y = newy;
            },
            Plane::XZ => {
                let newz = -1 * self.x;
                self.x = self.z;
                self.z = newz;
            },
        }
    }

    pub fn manhattan(&self, other: Self) -> Option<i64> {
        let out = other - *self;
        Some(out.x.abs() + out.y.abs() + out.z.abs())
    }
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

impl std::ops::Add for Beacon {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
