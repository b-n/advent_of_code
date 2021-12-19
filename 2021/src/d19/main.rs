use super::scanner::Scanner;
use std::fs;
use std::path::Path;
use std::collections::HashSet;

pub fn run() {
    let path = Path::new("./input/19_basic");

    println!("Part 1: {}", p01(path).unwrap());
    //println!("Part 2: {}", p02(path).unwrap());
}

fn p01(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;

    let mut scanners = raw_input
        .split("\n\n")
        .map(|x| x.parse::<Scanner>().unwrap())
        .collect::<Vec<Scanner>>();


    let mut to_match = vec![0];
    let mut candidates = HashSet::new();
    for i in 1..scanners.len() {
        candidates.insert(i);
    }

    println!("{:?}", to_match);

    // for each item to match
    // - attempt matching to each candidate
    // - for each matched candidate, they are now available to match (so we don't rotate them)
    // - remove the to_match from the candidates, we're matched
    while let Some(s1) = to_match.pop() {
        println!("test: {}", s1);
        let mut new_candidates = vec![];
        for s2 in candidates.iter() {
            if attempt_match(&mut scanners, s1, *s2)? {
                new_candidates.push(*s2); 
            } 
        }
        for c in new_candidates.iter() {
            to_match.push(*c);
        }
        candidates.remove(&s1); 
    }

    for scanner in scanners.iter() {
        println!("{} {:?}", scanner.name, scanner.matches);
    }

    Some(0)
}

fn attempt_match(scanners: &mut Vec<Scanner>, index1: usize, index2: usize) -> Option<bool> {
    match get_two_mut(scanners, index1, index2) {
        Some((s1, s2)) => {
            while let Some(matched) = s1.has_match(s2) {
                if matched {
                    s1.add_matches(&s2.name);
                    s2.add_matches(&s1.name);
                    s2.stop_rotation();
                    return Some(true);
                }
                if s2.rotate() == None {
                    break;
                }
            }
        },
        None => ()
    }
    Some(false)
}

fn get_two_mut<T>(slice: &mut [T], index1: usize, index2: usize) -> Option<(&mut T, &mut T)> {
    if index1 == index2 {
        None
    } else {
        let mut iter = slice.iter_mut();
        if index1 <= index2 {
            Some((iter.nth(index1).unwrap(), iter.nth(index2 - index1 - 1).unwrap()))
        } else {
            let second = iter.nth(index2).unwrap();
            Some((iter.nth(index1 - index2 - 1).unwrap(), second))
        }
    }
}
