
// Generics = Fun
// Here we are saying that T has some properties:
//  - PartialOrd - It can be compared
//  - Copy - We can get the associated value of it (e.g. we make a copy)
//  - Sub - We can subtract it, and output the same type
#![allow(dead_code)]
pub fn abs_diff<T: std::cmp::PartialOrd + Copy + std::ops::Sub<Output = T>>(left: &T, right: &T) -> T {
    if left > right {
        *left - *right
    } else {
        *right - *left
    }
}
