use crate::d11::point3d::Point3d;
use std::fmt;

pub struct Chart {
    points: Vec<Vec<Point3d>>,
    y_max: usize,
    x_max: usize,
}

impl Chart {
    pub fn new(points: Vec<Vec<Point3d>>) -> Self {
        let y_max = points.len();
        let x_max = points[0].len();
        Self {
            points: points,
            y_max,
            x_max,
        }
    }

    pub fn from_2d_vec(input: &Vec<Vec<usize>>) -> Self {
        Self::new(
            input
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(x, z)| Point3d { x, y, z: *z })
                        .collect::<Vec<Point3d>>()
                })
                .collect::<Vec<Vec<Point3d>>>(),
        )
    }

    pub fn at_pos(&mut self, x: usize, y: usize) -> Option<&mut Point3d> {
        match (x, y) {
            (_, y) if y >= self.y_max => None,
            (x, _) if x >= self.x_max => None,
            _ => Some(&mut self.points[y][x]),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Point3d> {
        self.points.iter().map(|l| l.iter()).flat_map(|r| r)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Point3d> {
        self.points.iter_mut().map(|l| l.iter_mut()).flat_map(|r| r)
    }

    pub fn inc(&mut self, i: usize) {
        for p in self.iter_mut() {
            p.z += i;
        }
    }

    pub fn reset<F>(&mut self, filter: F) -> usize
    where
        F: FnMut(&&mut Point3d) -> bool,
    {
        self.iter_mut().filter(filter).map(|p| p.z = 0).count()
    }
}

impl fmt::Display for Chart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        for l in self.points.iter() {
            for v in l.iter() {
                output += &format!("{: >3}", v.z);
            }
            output += "\n";
        }

        write!(f, "{}", output)
    }
}
