#![allow(dead_code)]
pub struct MinMax<T> {
    pub min: T,
    pub max: T,
}

pub fn min_max<T: std::cmp::PartialOrd + Copy>(v: &[T]) -> MinMax<T> {
    let mut min = &v[0];
    let mut max = &v[0];

    for val in v.iter() {
        if *val < *min {
            min = val;
        }
        if *val > *max {
            max = val;
        }
    }

    MinMax {
        min: *min,
        max: *max,
    }
}
