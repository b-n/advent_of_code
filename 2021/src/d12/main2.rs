use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

// Learnings:
// - see main.rs
// - Apparently doing everything with primitives actually makes things a whole bunch faster
//
// Rust Enjoyment factor [+++-------] <-- +1 from main.rs - I read up why linked lists are hard and
// how technically the below is "more optimal" :shrug:

pub fn run() {
    let path = Path::new("./input/12");

    println!("Part 1: {}", p01(path).unwrap());
    println!("Part 2: {}", p02(path).unwrap());
}

fn p01(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;

    let mut node_map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in raw_input.split("\n") {
        if line.is_empty() {
            break;
        }
        populate_node_map(&line, &mut node_map);
    }

    let mut all_paths = HashSet::new();

    traverse(
        &String::from("start"),
        &node_map,
        "start",
        "start",
        &mut all_paths,
    );

    Some(all_paths.len())
}

fn p02(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;

    let mut node_map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in raw_input.split("\n") {
        if line.is_empty() {
            break;
        }
        populate_node_map(&line, &mut node_map);
    }

    let mut all_paths = HashSet::new();

    traverse(
        &String::from("start"),
        &node_map,
        "start",
        "",
        &mut all_paths,
    );

    Some(all_paths.len())
}

fn traverse(
    from: &str,
    map: &HashMap<&str, Vec<&str>>,
    path: &str,
    smol_node: &str,
    all_paths: &mut HashSet<String>,
) {
    for next_node in map
        .get(from)
        .unwrap()
        .iter()
        .filter(|&next| next != &"start")
        .filter(|&next| {
            if (*next).chars().all(|c| c.is_ascii_lowercase()) {
                let allowed_times = if &smol_node == next { 2 } else { 1 };
                path.matches(next).count() < allowed_times
            } else {
                true
            }
        })
    {
        let next_path = format!("{}{}", path, next_node);

        if all_paths.contains(&next_path) {
            continue;
        }

        if next_node == &"end" {
            all_paths.insert(next_path);
            continue;
        }

        traverse(
            &next_node,
            map,
            next_path.as_str(),
            smol_node.clone(),
            all_paths,
        );
        if smol_node.is_empty() && next_node.chars().all(|c| c.is_ascii_lowercase()) {
            traverse(&next_node, map, next_path.as_str(), next_node, all_paths);
        }
    }
}

fn populate_node_map<'a>(input: &'a str, map: &mut HashMap<&'a str, Vec<&'a str>>) {
    let (left, right) = node_points(input);

    let left_node = map.entry(left).or_insert(vec![]);
    left_node.push(right);
    let right_node = map.entry(right).or_insert(vec![]);
    right_node.push(left);
}

fn node_points(input: &str) -> (&str, &str) {
    let mut parts = input.split("-");
    (parts.next().unwrap(), parts.next().unwrap())
}
