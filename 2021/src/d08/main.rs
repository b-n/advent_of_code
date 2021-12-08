use crate::utils::file;
use std::collections::{HashMap, HashSet};
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

    raw_input
        .map(|l| file::line_as_str(l).unwrap())
        .map(|l| {
            let (unique_signals, output_values) = parse_line(l);

            signal_value(&unique_signals, &output_values)
        })
        .sum()
}

type Signal = HashSet<char>;

fn signal_value(unique_signals: &Vec<String>, output_values: &Vec<String>) -> Option<usize> {
    let signals = unique_signals
        .iter()
        .map(|s| set_from_iter(&mut s.chars()))
        .collect::<Vec<Signal>>();

    let signals_by_len: HashMap<usize, Vec<&Signal>> =
        signals.iter().fold(HashMap::new(), |mut acc, i| {
            acc.entry(i.len()).or_insert(vec![]);
            acc.get_mut(&i.len()).unwrap().push(i);
            acc
        });

    let d1 = signals_by_len.get(&2)?.first()?;
    let d7 = signals_by_len.get(&3)?.first()?;
    let d4 = signals_by_len.get(&4)?.first()?;
    let d8 = signals_by_len.get(&7)?.first()?;

    // a = diff between 7 and 1
    let a_char = *d7.difference(d1).collect::<Vec<&char>>()[0];

    // find 9/g = hashset len 6 and has 4 + a
    let mut check_g = (*d4).clone();
    check_g.insert(a_char);
    let d9 = signals_by_len
        .get(&6)?
        .iter()
        .filter(|sh| sh.difference(&check_g).count() == 1)
        .next()?;
    let g_char = **d9.difference(&check_g).collect::<Vec<&char>>().first()?;

    // find 3/d = hashset len 5 and has 1 + a + g
    let mut check_d = (*d1).clone();
    check_d.insert(g_char);
    check_d.insert(a_char);
    let d3 = signals_by_len
        .get(&5)?
        .iter()
        .filter(|sh| sh.difference(&check_d).count() == 1)
        .next()?;

    let mut check0 = (*d7).clone();
    check0.insert(g_char);
    let d0 = signals_by_len
        .get(&6)?
        .iter()
        .filter(|sh| sh != &d9 && sh.difference(&check_d).collect::<Vec<&char>>().len() == 2)
        .next()?;

    let d6 = signals_by_len
        .get(&6)?
        .iter()
        .filter(|s| s != &d9 && s != &d0)
        .next()?;

    let d2 = signals_by_len
        .get(&5)?
        .iter()
        .filter(|s| s != &d3 && s.difference(d6).collect::<Vec<&char>>().len() == 1)
        .next()?;

    let d5 = signals_by_len
        .get(&5)?
        .iter()
        .filter(|s| s != &d3 && s != &d2)
        .next()?;

    let digits = vec![*d0, *d1, *d2, *d3, *d4, *d5, *d6, *d7, *d8, *d9];

    let base: usize = 10;

    // For each output value, get the HashSet, Find it's position in the digits vector (position =
    // digit), and then get the output value with some math::pow magic
    Some(
        output_values
            .iter()
            .map(|s| set_from_iter(&mut s.chars()))
            .map(|v| digits.iter().position(|&d| *d == v))
            .enumerate()
            .map(|(i, v)| base.pow(3 - i as u32) * v.unwrap())
            .sum(),
    )
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
