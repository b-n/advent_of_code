use crate::types::point::Point;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    #![allow(dead_code)]
    pub fn is_straight(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    // ouch :-(. I tried thinking of a way to sort lines, but brain no work
    pub fn is_point_in_line(&self, p: &Point) -> bool {
        if self.is_horizontal() {
            if self.start.y != p.y {
                return false
            }
            
            let (min, max) = if self.start.x < self.end.x {
                (self.start.x, self.end.x)
            } else {
                (self.end.x, self.start.x)
            };

            p.x >= min && p.x <= max
        } else if self.is_vertical() {
            if self.start.x != p.x {
                return false
            }
            
            let (min, max) = if self.start.y < self.end.y {
                (self.start.y, self.end.y)
            } else {
                (self.end.y, self.start.y)
            };

            p.y >= min && p.y <= max
        } else {
            false
        }
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" -> ").collect::<Vec<&str>>();

        let start = parts[0].parse::<Point>().expect("Failed to parse point");
        let end = parts[1].parse::<Point>().expect("Failed to parse point");

        Ok(Line { start, end })
    }
}
