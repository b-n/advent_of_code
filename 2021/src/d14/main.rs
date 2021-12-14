use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Learnings
// - (From yesterday refactor) avoid &String and String by assigning variables so the borrow
//   checker knows where they came from

pub fn run() {
    let path = Path::new("./input/14");

    println!("Part 1: {}", p01(path, 10).unwrap());
    println!("Part 2: {}", p02(path, 40).unwrap());
}

fn p01(p: &Path, steps: usize) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;
    let (initial_points, mutations) = parse_input(&raw_input)?;

    let mut next: String = String::from(initial_points);
    for _ in 0..steps {
        next = step(&next, &mutations)?;
    }

    let char_occurences = next.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    Some(char_occurences.values().max()? - char_occurences.values().min()?)
}

fn p02(p: &Path, steps: usize) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;
    let (initial_points, raw_mutations) = parse_input(&raw_input)?;

    let net_effect_mutations = raw_mutations
        .iter()
        .map(|(&k, v)| {
            (
                String::from(k),
                vec![format!("{}{}", &k[0..=0], v), format!("{}{}", v, &k[1..=1])],
            )
        })
        .collect::<HashMap<String, Vec<String>>>();

    // this caused me hassles, I originally didn't count duplicates in the input. whoops!
    let mut pair_counts = (0..(initial_points.len() - 1)).fold(HashMap::new(), |mut acc, i| {
        let pair = &initial_points[i..=(i + 1)];
        *acc.entry(pair).or_insert(0) += 1;
        acc
    });

    for _ in 0..steps {
        pair_counts = better_step(&mut pair_counts, &net_effect_mutations)?;
    }

    let mut char_occurences = HashMap::new();
    for (k, v) in pair_counts.iter() {
        *char_occurences.entry(&k[0..=0]).or_insert(0) += v;
        *char_occurences.entry(&k[1..=1]).or_insert(0) += v;
    }

    Some((char_occurences.values().max()? - char_occurences.values().min()?) / 2)
}

fn step<'a>(input: &'a str, mutations: &HashMap<&str, &str>) -> Option<String> {
    let mut next_input = format!("{}", &input[0..1]);
    for i in 0..(input.len() - 1) {
        let next_char = mutations.get(&input[i..i + 2])?;
        next_input = format!("{}{}{}", next_input, next_char, &input[i + 1..i + 2]);
    }

    Some(next_input)
}

fn better_step<'a>(
    pair_counts: &mut HashMap<&'a str, usize>,
    effect_mutations: &'a HashMap<String, Vec<String>>,
) -> Option<HashMap<&'a str, usize>> {
    let mut new_counts: HashMap<&'a str, usize> = HashMap::new();
    for (pair, count) in pair_counts.iter() {
        for new_pair in effect_mutations.get(*pair)?.iter() {
            *new_counts.entry(new_pair).or_insert(0) += count;
        }
    }

    Some(new_counts)
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
