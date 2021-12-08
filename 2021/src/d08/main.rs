use crate::utils::file;
use std::collections::HashSet;
use std::path::Path;

// Learnings:
//  - We're at that pen and paper stage of aoc
//  - HashSet equality is something pretty damn cool
//  - Doing this stuff in rust is.... not pretty
//
//  Rust enjoyment factor [++++------]

pub fn run() {
    let path = Path::new("./input/08");

    println!("Part 1: {}", p01(path).unwrap());
    println!("Part 2: {}", p02(path).unwrap());
}

fn p01(p: &Path) -> Option<usize> {
    let raw_input = file::read_to_lines(p);

    let easy_number_lens: Vec<usize> = vec![2, 3, 4, 7];

    Some(
        raw_input
            .map(|l| file::line_as_str(l).unwrap())
            .flat_map(|l| {
                let (_, output_values) = parse_line(l);

                output_values
            })
            .filter(|v| easy_number_lens.contains(&v.len()))
            .collect::<Vec<String>>()
            .len(),
    )
}

fn p02(p: &Path) -> Option<usize> {
    let raw_input = file::read_to_lines(p);

    Some(
        raw_input
            .map(|l| file::line_as_str(l).unwrap())
            .map(|l| {
                let (unique_signals, output_values) = parse_line(l);

                signal_value(&unique_signals, &output_values)
            })
            .sum(),
    )
}

fn signal_value(unique_signals: &Vec<String>, output_values: &Vec<String>) -> usize {
    let signal_hashsets = unique_signals
        .iter()
        .map(|s| set_from_iter(&mut s.chars()))
        .collect::<Vec<HashSet<char>>>();

    let d1: &HashSet<char> = find_in_vec_by_len(&signal_hashsets, 2).unwrap();
    let d7: &HashSet<char> = find_in_vec_by_len(&signal_hashsets, 3).unwrap();
    let d4: &HashSet<char> = find_in_vec_by_len(&signal_hashsets, 4).unwrap();
    let d8: &HashSet<char> = find_in_vec_by_len(&signal_hashsets, 7).unwrap();

    // a = diff between 7 and 1
    let a_char = *(d7.difference(d1).collect::<Vec<&char>>()[0]);
    // these all get overwritten
    let mut g_char = 'g';
    let mut d_char = 'd';

    // find 9/g = hashset len 6 and has 4 + a
    let mut check_g = d4.clone();
    check_g.insert(a_char);
    let d9 = (*signal_hashsets)
        .iter()
        .filter(|s| s.len() == 6)
        .filter(|sh| {
            let diff = sh.difference(&check_g).collect::<Vec<&char>>();
            if diff.len() == 1 {
                g_char = *(diff[0]);
                true
            } else {
                false
            }
        })
        .next()
        .unwrap();

    // find 3/d = hashset len 5 and has 1 + a + g
    let mut check_d = d1.clone();
    check_d.insert(g_char);
    check_d.insert(a_char);
    let d3 = (*signal_hashsets)
        .iter()
        .filter(|s| s.len() == 5)
        .filter(|sh| {
            let diff = sh.difference(&check_d).collect::<Vec<&char>>();
            if diff.len() == 1 {
                d_char = *(diff[0]);
                true
            } else {
                false
            }
        })
        .next()
        .unwrap();

    let mut check0 = d7.clone();
    check0.insert(g_char);
    let d0 = (*signal_hashsets)
        .iter()
        .filter(|s| s.len() == 6 && s != &d9)
        .filter(|sh| sh.difference(&check_d).collect::<Vec<&char>>().len() == 2)
        .next()
        .unwrap();

    let d6 = (*signal_hashsets)
        .iter()
        .filter(|s| s.len() == 6 && s != &d9 && s != &d0)
        .next()
        .unwrap();

    let d2 = (*signal_hashsets)
        .iter()
        .filter(|s| s.len() == 5 && s != &d3)
        .filter(|s| s.difference(d6).collect::<Vec<&char>>().len() == 1)
        .next()
        .unwrap();

    let d5 = (*signal_hashsets)
        .iter()
        .filter(|s| s.len() == 5 && s != &d3 && s != &d2)
        .next()
        .unwrap();

    let digits = vec![d0, d1, d2, d3, d4, d5, d6, d7, d8, d9];

    let base: usize = 10;

    // For each output value, get the HashSet, Find it's position in the digits vector (position =
    // digit), and then get the output value with some math::pow magic
    output_values
        .iter()
        .map(|s| set_from_iter(&mut s.chars()))
        .map(|v| {
            let pos = digits.iter().position(|&d| d == &v).unwrap();
            pos
        })
        .enumerate()
        .map(|(i, v)| base.pow(3 - i as u32) * v)
        .sum()
}

// I'm pretty proud of this one. I didn't even do much googling
// What's it do? create a HashSet from an iterator
fn set_from_iter<T, I>(it: &mut I) -> HashSet<T>
where
    T: std::hash::Hash + Eq,
    I: Iterator<Item = T>,
{
    let mut res = HashSet::new();
    while let Some(value) = it.next() {
        res.insert(value);
    }
    res
}

fn find_in_vec_by_len(vec: &Vec<HashSet<char>>, len: usize) -> Option<&HashSet<char>> {
    (*vec).iter().filter(|s| s.len() == len).next()
}

fn parse_line(s: String) -> (Vec<String>, Vec<String>) {
    let parts = s
        .split(" | ")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();
    (
        parts[0].split(" ").map(|s| String::from(s)).collect(),
        parts[1].split(" ").map(|s| String::from(s)).collect(),
    )
}
