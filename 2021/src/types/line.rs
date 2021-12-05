use crate::types::point::Point;
use std::num::ParseIntError;
use std::str::FromStr;

pub enum LineType {
    Horizontal,
    Vertical,
    Diagonal,
}

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

    pub fn line_type(&self) -> LineType {
        if self.is_horizontal() {
            LineType::Horizontal
        } else if self.is_vertical() {
            LineType::Vertical
        } else {
            LineType::Diagonal
        }
    }

    pub fn points(&self) -> Option<Vec<Point>> {
        let (start_x, start_y) = (self.start.x as i64, self.start.y as i64);
        let (end_x, end_y) = (self.end.x as i64, self.end.y as i64);

        let x_diff = end_x - start_x;
        let x_step = if x_diff == 0 {
            0
        } else {
            x_diff / x_diff.abs()
        };

        let y_diff = end_y - start_y;
        let y_step = if y_diff == 0 {
            0
        } else {
            y_diff / y_diff.abs()
        };

        let distance = std::cmp::max(x_diff.abs(), y_diff.abs());
        //println!("Start: {} {}, End: {} {}, distance: {}, xinc: {}, yinc: {}", start_x, start_y, end_x, end_y, distance, x_step, y_step);

        Some(
            (0..=distance)
                .map(|i| {
                    let x = start_x + x_step * i;
                    let y = start_y + y_step * i;
                    Point {
                        x: x as usize,
                        y: y as usize,
                    }
                })
                .collect::<Vec<Point>>(),
        )
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" -> ").collect::<Vec<&str>>();

        let start = parts[0].parse::<Point>()?;
        let end = parts[1].parse::<Point>()?;

        Ok(Line { start, end })
    }
}
