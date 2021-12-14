use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Learnings
// - (From yesterday refactor) avoid &String and String by assigning variables so the borrow
//   checker knows where they came from

pub fn run() {
    let path = Path::new("./input/14");

    println!("Part 1: {}", p01(path).unwrap());
    //println!("Part 2: {}", p02(path).unwrap());
}

fn p01(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;
    let (initial_points, mutations) = parse_input(&raw_input)?;

    let mut next: String = String::from(initial_points);
    for i in 0..10 {
        next = step(&next, &mutations)?;
        println!("{}", i);
    }

    let char_occurences = next.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    Some(char_occurences.values().max()? - char_occurences.values().min()?)
}

//fn p02(p: &Path) -> Option<usize> {
    //let (initial_points, folds) = parse_input(p)?;

    //Some(0)
//}

fn step<'a>(input: &'a str, mutations: &HashMap<&str, &str>) -> Option<String> {
    let mut next_input = format!("{}", &input[0..1]);
    for i in 0..(input.len() - 1) {
        let next_char = mutations.get(&input[i..i + 2])?;
        next_input = format!("{}{}{}", next_input, next_char, &input[i + 1..i + 2]);
    }

    Some(next_input)
}

fn parse_input(input: &str) -> Option<(&str, HashMap<&str, &str>)> {
    let mut parts = input.split("\n\n");

    let template = parts.next()?;

    let instructions = parts
        .next()?
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let mut parts = x.split(" -> ");
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect::<HashMap<&str, &str>>();

    Some((template, instructions))
}
