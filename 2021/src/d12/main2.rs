use crate::utils::file;
use std::collections::{HashMap, HashSet};
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
    let lines = file::read_to_lines(p);

    let mut node_map: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let str_value = file::line_as_str(line).ok()?;
        populate_node_map(str_value, &mut node_map);
    }

    let mut all_paths = HashSet::new();
    
    traverse(
        &String::from("start"),
        &node_map,
        String::from("start"),
        String::from("start"),
        &mut all_paths,
    );

    Some(all_paths.len())
}

fn p02(p: &Path) -> Option<usize> {
    let lines = file::read_to_lines(p);

    let mut node_map: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let str_value = file::line_as_str(line).ok()?;
        populate_node_map(str_value, &mut node_map);
    }

    let mut all_paths = HashSet::new();

    traverse(
        &String::from("start"),
        &node_map,
        String::from("start"),
        String::new(),
        &mut all_paths,
    );

    Some(all_paths.len())
}

fn traverse(
    from: &String,
    map: &HashMap<String, Vec<String>>,
    path: String,
    smol_node: String,
    all_paths: &mut HashSet<String>,
) {
    for next_node in map.get(from).unwrap()
        .iter()
        .filter(|&next| next != "start")
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

        if next_node == "end" {
            all_paths.insert(next_path);
            continue;
        }

        traverse(&next_node, map, next_path.clone(), smol_node.clone(), all_paths);
        if smol_node.is_empty() && next_node.chars().all(|c| c.is_ascii_lowercase()) {
            traverse(&next_node, map, next_path.clone(), next_node.clone(), all_paths);
        }
    }
}

fn populate_node_map(input: String, map: &mut HashMap<String, Vec<String>>) {
    let (left, right) = node_points(input);

    let left_node = map.entry(left.clone()).or_insert(vec![]);
    left_node.push(right.clone());
    let right_node = map.entry(right.clone()).or_insert(vec![]);
    right_node.push(left.clone());
}

fn node_points(input: String) -> (String, String) {
    let parts = input.split("-").collect::<Vec<&str>>();
    (String::from(parts[0]), String::from(parts[1]))
}
