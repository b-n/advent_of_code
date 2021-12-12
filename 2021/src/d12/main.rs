use super::node::Node;
use crate::utils::file;
use std::collections::{HashMap, HashSet};
use std::path::Path;

// Learnings:
// - Self-referential objects in rust are not really a thing apparently (I should learn Box<T>)
//   - But really, it's a graph, there should be an easy way to do this
// - Read the description of the task better (second task is one cave twice etc)
// - Just dedupe with hashsets - your algo sucks, so just brute it 
//
// Rust Enjoyment factor [++--------] <-- -2 points, serious, lifetimes are hard when doing linked
// lists

pub fn run() {
    let path = Path::new("./input/12");

    println!("Part 1: {}", p01(path).unwrap());
    println!("Part 2: {}", p02(path).unwrap());
}

fn p01(p: &Path) -> Option<usize> {
    let lines = file::read_to_lines(p);

    let start = Node::new(String::from("start"));
    let end = Node::new(String::from("end"));

    let mut node_map: HashMap<String, Node> = HashMap::new();
    node_map.insert(start.name.clone(), start);
    node_map.insert(end.name.clone(), end);

    for line in lines {
        let str_value = file::line_as_str(line).ok()?;
        populate_node_map(str_value, &mut node_map);
    }

    let mut cache = HashSet::new();
    let paths = traverse(
        &String::from("start"),
        &node_map,
        String::from("start"),
        String::from("start"),
        &mut cache,
    );

    paths
}

fn p02(p: &Path) -> Option<usize> {
    let lines = file::read_to_lines(p);

    let start = Node::new(String::from("start"));
    let end = Node::new(String::from("end"));

    let mut node_map: HashMap<String, Node> = HashMap::new();
    node_map.insert(start.name.clone(), start);
    node_map.insert(end.name.clone(), end);

    for line in lines {
        let str_value = file::line_as_str(line).ok()?;
        populate_node_map(str_value, &mut node_map);
    }

    let mut cache = HashSet::new();

    Some(
        traverse(
            &String::from("start"),
            &node_map,
            String::from("start"),
            String::new(),
            &mut cache,
        )
        .unwrap(),
    )
}

fn traverse(
    from: &String,
    map: &HashMap<String, Node>,
    travelled_path: String,
    smol_node: String,
    cache: &mut HashSet<String>,
) -> Option<usize> {
    let node = map.get(from)?;

    let connections = node
        .connections
        .iter()
        .map(|c| map.get(c).unwrap())
        .filter(|next| next.name != "start")
        .filter(|next| {
            if next.is_smol {
                let allowed_times = if smol_node == next.name { 2 } else { 1 };
                travelled_path.matches(&next.name).count() < allowed_times
            } else {
                true
            }
        })
        .collect::<Vec<&Node>>();

    if connections.len() == 0 {
        Some(0)
    } else {
        Some(
            connections
                .iter()
                .map(|next| {
                    let current_path = String::from(format!("{}->{}", travelled_path, next.name));
                    if cache.contains(&current_path) {
                        // Hax. We have duplicate paths, so this just crushes them
                        0
                    } else if next.name == String::from("end") {
                        cache.insert(current_path.clone());
                        1
                    } else {
                        // this isn't obvious, but if we have a smol_node, we need to keep parsing
                        // If we don't have one, we should branch out and find from that smol_node
                        traverse(&next.name, map, current_path.clone(), smol_node.clone(), cache).unwrap()
                            + if smol_node.is_empty() {
                                traverse(&next.name, map, current_path.clone(), next.name.clone(), cache)
                                    .unwrap()
                            } else {
                                0
                            }
                    }
                })
                .sum(),
        )
    }
}

fn populate_node_map(input: String, map: &mut HashMap<String, Node>) {
    let (left, right) = node_points(input);

    // why so much clone? Well using the value once is fine (it takes ownership), but more than
    // once = many things owning the same variable = not good. And passing refs everywhere
    // would normally be fine, but rust doesn't like the idea of something outliving it's refs

    let left_node = map.entry(left.clone()).or_insert(Node::new(left.clone()));
    left_node.add_connection(right.clone());
    let right_node = map.entry(right.clone()).or_insert(Node::new(right.clone()));
    right_node.add_connection(left.clone());
}

fn node_points(input: String) -> (String, String) {
    let parts = input.split("-").collect::<Vec<&str>>();
    (String::from(parts[0]), String::from(parts[1]))
}
