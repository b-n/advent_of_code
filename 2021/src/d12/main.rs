use super::node::Node;
use super::node_path::NodePath;
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

    let mut node_map: HashMap<String, Node> = HashMap::new();

    for line in lines {
        let str_value = file::line_as_str(line).ok()?;
        populate_node_map(str_value, &mut node_map);
    }

    let start_node = node_map.get(&String::from("start"))?;
    let mut all_paths = HashSet::new();
    
    traverse(
        &start_node.name,
        &node_map,
        NodePath::new(vec![start_node.name.clone()]),
        String::from("start"),
        &mut all_paths,
    );

    Some(all_paths.len())
}

fn p02(p: &Path) -> Option<usize> {
    let lines = file::read_to_lines(p);

    let mut node_map: HashMap<String, Node> = HashMap::new();

    for line in lines {
        let str_value = file::line_as_str(line).ok()?;
        populate_node_map(str_value, &mut node_map);
    }

    let mut all_paths = HashSet::new();

    let start_node = node_map.get(&String::from("start"))?;
    traverse(
        &start_node.name,
        &node_map,
        NodePath::new(vec![start_node.name.clone()]),
        String::new(),
        &mut all_paths,
    );

    Some(all_paths.len())
}

fn traverse(
    from: &String,
    map: &HashMap<String, Node>,
    path: NodePath,
    smol_node: String,
    all_paths: &mut HashSet<NodePath>,
) {
    for next_node in map.get(from).unwrap()
        .connections
        .iter()
        .map(|c| map.get(c).unwrap())
        .filter(|next| next.name != "start")
        .filter(|next| {
            if next.is_smol {
                let allowed_times = if smol_node == next.name { 2 } else { 1 };
                path.count_node(next) < allowed_times
            } else {
                true
            }
        })
    {
        let next_path = NodePath::from(&path, next_node);
        if all_paths.contains(&next_path) {
            continue;
        }

        if next_node.is_end {
            all_paths.insert(next_path);
            continue;
        }

        traverse(&next_node.name, map, next_path.clone(), smol_node.clone(), all_paths);
        if next_node.is_smol && smol_node.is_empty() {
            traverse(&next_node.name, map, next_path.clone(), next_node.name.clone(), all_paths);
        }
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
