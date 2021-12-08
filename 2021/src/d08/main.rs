use crate::utils::file;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn run() {
    let path = Path::new("./input/08_example");

    //println!("Part 1: {}", p01(path).unwrap());
    println!("Part 2: {}", p02(path).unwrap());
}

// 0 : abcefg
// 1 : cf      *
// 2 : acdeg
// 3 : acdfg
// 4 : bcdf    *
// 5 : abdfg
// 6 : abdefg
// 7 : acf     *
// 8 : abcedfg *
// 9 : abcdfg

fn p01(p: &Path) -> Option<usize> {
    let raw_input = file::read_to_lines(p);

    //let counts: Vec<usize> = vec![0; 10];
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
    let mut signal_hashsets: Vec<HashSet<char>> = vec![];
    for s in unique_signals.iter() {
        let mut signal_hash = HashSet::new();
        for c in s.chars() {
            signal_hash.insert(c);
        }

        signal_hashsets.push(signal_hash)
    }

    println!("{:?}", signal_hashsets);

    let digit1: &HashSet<char> = find_in_vec_by_len(&signal_hashsets, 2).unwrap();
    let digit7: &HashSet<char> = find_in_vec_by_len(&signal_hashsets, 3).unwrap();
    let digit4: &HashSet<char> = find_in_vec_by_len(&signal_hashsets, 4).unwrap();
    let digit8: &HashSet<char> = find_in_vec_by_len(&signal_hashsets, 7).unwrap();
    //let mut digit9;

    // find a = diff between 7 and 1
    let a_char = *(digit7.difference(digit1).collect::<Vec<&char>>()[0]);
    // these all get overwritten
    let mut g_char = 'g';
    let mut d_char = 'd';

    // find 9/g = hashset len 6 and has 4 + a 
    let mut check_g = digit4.clone();
    check_g.insert(a_char);
    let digit9 = (*signal_hashsets).iter()
        .filter(|s| s.len() == 6)
        .filter(|sh| {
            let diff = sh.difference(&check_g).collect::<Vec<&char>>();
            if diff.len() == 1 {
                g_char = *(diff[0]);
                true
            } else {
                false
            }
        }).next().unwrap();


    // find 3/d = hashset len 5 and has 1 + a + g
    let mut check_d = digit1.clone();
    check_d.insert(g_char);
    check_d.insert(a_char);
    let digit3 = (*signal_hashsets).iter()
        .filter(|s| s.len() == 5)
        .filter(|sh| {
            let diff = sh.difference(&check_d).collect::<Vec<&char>>();
            if diff.len() == 1 {
                d_char = *(diff[0]);
                true
            } else {
                false
            }
        }).next().unwrap();
     
    // find 6/be = 

    println!("{} {} {}", a_char, g_char, d_char);

    println!("{:?} {:?}", unique_signals, a_char);

    0
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
