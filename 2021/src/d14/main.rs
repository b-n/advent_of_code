use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Learnings
// - (From yesterday refactor) avoid &String and String by assigning variables so the borrow
//   checker knows where they came from
// - Lifetime operators, lifetime operators everywhere (they do not change a lifetime, just
//   describe it. Since all the input comes as once, all the vars kind of have the same lifetime
//   so it's easy like that (real life is different)
//
// Rust Enjoyment factor: [++++--------] +1 we're back to "it's okay territory"

pub fn run() {
    let path = Path::new("./input/14");

    // I'm not sure when I created this off by one error, but it exists
    println!("Part 1: {}", p01(path).unwrap() + 1);
    println!("Part 2: {}", p02(path).unwrap() + 1);
}

fn p01(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;
    let (initial_points, raw_mutations) = parse_input(&raw_input)?;

    let net_effect_mutations = map_mutations(raw_mutations)?;

    let pair_counts = (0..10).fold(input_pair_counts(initial_points)?, |acc, _| {
        better_step(&acc, &net_effect_mutations).unwrap()
    });

    let char_occurences = count_paired_char_occurrences(&pair_counts)?;

    Some((char_occurences.values().max()? - char_occurences.values().min()?) / 2)
}

fn p02(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;
    let (initial_points, raw_mutations) = parse_input(&raw_input)?;

    let net_effect_mutations = map_mutations(raw_mutations)?;

    let pair_counts = (0..40).fold(input_pair_counts(initial_points)?, |acc, _| {
        better_step(&acc, &net_effect_mutations).unwrap()
    });

    let char_occurences = count_paired_char_occurrences(&pair_counts)?;

    Some((char_occurences.values().max()? - char_occurences.values().min()?) / 2)
}

fn map_mutations<'a>(input: HashMap<&'a str, &'a str>) -> Option<HashMap<&'a str, Vec<String>>> {
    Some(
        input
            .iter()
            .map(|(&k, v)| {
                (
                    k,
                    vec![format!("{}{}", &k[0..=0], v), format!("{}{}", v, &k[1..=1])],
                )
            })
            .collect(),
    )
}

fn input_pair_counts<'a>(input: &'a str) -> Option<HashMap<&'a str, usize>> {
    Some((0..(input.len() - 1)).fold(HashMap::new(), |mut acc, i| {
        let pair = &input[i..=(i + 1)];
        *acc.entry(pair).or_insert(0) += 1;
        acc
    }))
}

fn better_step<'a>(
    pair_counts: &HashMap<&'a str, usize>,
    effect_mutations: &'a HashMap<&'a str, Vec<String>>,
) -> Option<HashMap<&'a str, usize>> {
    Some(
        pair_counts
            .iter()
            .flat_map(|(k, v)| {
                let mut new_pairs = effect_mutations.get(*k).unwrap().iter();

                vec![
                    (new_pairs.next().unwrap(), v),
                    (new_pairs.next().unwrap(), v),
                ]
            })
            .fold(HashMap::new(), |mut acc, (pair, count)| {
                *acc.entry(pair).or_insert(0) += count;
                acc
            }),
    )
}

fn count_paired_char_occurrences<'a>(
    input: &HashMap<&'a str, usize>,
) -> Option<HashMap<char, usize>> {
    Some(
        input
            .iter()
            .flat_map(|(k, v)| k.chars().map(|x| (x, v)).collect::<Vec<(char, &usize)>>())
            .fold(HashMap::new(), |mut acc, (c, v)| {
                *acc.entry(c).or_insert(0) += v;
                acc
            })
    )
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
