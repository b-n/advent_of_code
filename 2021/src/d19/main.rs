use super::scanner::Scanner;
use super::beacon::Beacon;
use std::fs;
use std::path::Path;
use std::collections::{HashSet, HashMap};

pub fn run() {
    let path = Path::new("./input/19");

    println!("Part 1: {}", p01(path).unwrap());
    println!("Part 2: {}", p02(path).unwrap());
}

fn p01(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;

    let mut scanners = raw_input
        .split("\n\n")
        .map(|x| x.parse::<Scanner>().unwrap())
        .map(|x| (x.id, x))
        .collect::<HashMap<usize, Scanner>>();

    build_chart(&mut scanners);

    let mut all_beacons: HashSet<Beacon> = HashSet::new();
    for scanner in scanners.values() {
        all_beacons = all_beacons.union(&scanner.beacons())
            .map(|&x| x)
            .collect();
    }

    Some(all_beacons.len())
}

fn p02(p: &Path) -> Option<i64> {
    let raw_input = fs::read_to_string(p).ok()?;

    let mut scanners = raw_input
        .split("\n\n")
        .map(|x| x.parse::<Scanner>().unwrap())
        .map(|x| (x.id, x))
        .collect::<HashMap<usize, Scanner>>();

    build_chart(&mut scanners);

    let mut all_beacons: HashSet<Beacon> = HashSet::new();
    for scanner in scanners.values() {
        all_beacons = all_beacons.union(&scanner.beacons())
            .map(|&x| x)
            .collect();
    }

    let mut max_manhattan = 0;
    for i in 0..scanners.len() {
        for j in 0..scanners.len() {
            let p1 = scanners.get(&i)?.center;
            let p2 = scanners.get(&j)?.center;
            let dist = p1.manhattan(p2)?;
            if dist > max_manhattan {
                max_manhattan = dist;
            }
        }
    }

    Some(max_manhattan)
}


fn build_chart(scanners: &mut HashMap<usize, Scanner>) -> Option<()> {
    let mut to_match = vec![0];
    let mut candidates = HashSet::new();
    for i in 1..scanners.len() {
        candidates.insert(i);
    }

    while let Some(s1) = to_match.pop() {
        let mut new_candidates = vec![];
        for s2 in candidates.iter() {
            if attempt_match(scanners, s1, *s2)? {
                new_candidates.push(*s2); 
            } 
        }
        for c in new_candidates.iter() {
            to_match.push(*c);
        }
        candidates.remove(&s1); 
    }

    scanners.get_mut(&0)?.set_center(Beacon { x:0 ,y: 0,z: 0 });
    let mut center = vec![0];

    while let Some(reference) = center.pop() {
        let mut next = HashSet::new();
        let mut matches = scanners.get(&reference)?.matches.clone();
        while let Some(to_center) = matches.pop() {
            let (s1, s2) = get_scanners(scanners.iter_mut(), reference, to_center)?;
            if s2.has_center {
                continue;
            }
            next.insert(to_center);

            let matching = s1.matching_vectors(s2);
            let v = matching.iter().next()?;
            let p1 = *s1.vectors.get(v)?;
            let p2 = *s2.vectors.get(v)?;
            s2.set_center(p1 - p2 + s1.center);
        }
        for n in next {
            center.push(n);
        }
    }

    Some(())
}

fn attempt_match(scanners: &mut HashMap<usize, Scanner>, index1: usize, index2: usize) -> Option<bool> {
    match get_scanners(scanners.iter_mut(), index1, index2) {
        Some((s1, s2)) => {
            while let Some(matched) = s1.has_match(s2) {
                if matched {
                    s1.add_matches(s2.id);
                    s2.add_matches(s1.id);
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

fn get_scanners<'a, I>(it: I, id1: usize, id2: usize) -> Option<(&'a mut Scanner, &'a mut Scanner)>
where
    I: Iterator<Item = (&'a usize, &'a mut Scanner)>
{
    if id1 == id2 {
        return None;
    }

    let mut items = it.filter(|(&x, _)| x == id1 || x == id2);
    let (i1, item1) = items.next().unwrap();
    let (_, item2) = items.next().unwrap();

    if i1 == &id1 {
        Some((item1, item2))
    } else {
        Some((item2, item1))
    }
}
