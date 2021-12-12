use super::node::Node;
use crate::utils::file;
use std::collections::HashMap;
use std::path::Path;

pub fn run() {
    let path = Path::new("./input/12");

    println!("Part 1: {}", p01(path).unwrap());
    //println!("Part 2: {}", p02(path).unwrap());
}

fn p01<'a>(p: &Path) -> Option<usize> {
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

    let paths = traverse(&String::from("start"), &node_map, String::from("start"));

    paths
}

//fn p02(p: &Path) -> Option<usize> {
    //let lines = file::read_to_lines(p);
    //Some(0)
//}

fn traverse(from: &String, map: &HashMap<String, Node>, travelled_path: String) -> Option<usize> {
    let node = map.get(from)?;

    if node.name == String::from("end") {
        return Some(1);
    }

    let connections = node
        .connections
        .iter()
        .map(|c| map.get(c).unwrap())
        .filter(|next| !(next.is_smol && travelled_path.contains(&next.name)))
        //.filter(|next| !travelled_path.contains(&String::from(format!("{}->{}", node.name, next.name)))) //apparently i don't need this :huh_wut:
        .collect::<Vec<&Node>>();

    if connections.len() == 0 {
        Some(0)
    } else {
        Some(
            connections
                .iter()
                .map(|c| {
                    let current_path = String::from(format!("{}->{}", travelled_path, c.name));
                    traverse(&c.name, map, current_path.clone()).unwrap()
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
