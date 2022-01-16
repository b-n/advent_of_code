use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn run() {
    let path = Path::new("./input/14_example");
    //let path = Path::new("./input/14");

    let steps = 10;
    //let steps = 40;

    println!("Naive: {}", naive(path, steps).unwrap() + 1);
    println!("Better: {}", better(path, steps).unwrap() + 1);
}

fn naive(p: &Path, steps: usize) -> Option<usize> {
    // reading and parsing
    let raw_input = fs::read_to_string(p).ok()?;
    let (template, mutations) = parse_input(&raw_input)?;

    let mut result: String = String::from(template);
    for i in 0..steps {
        // reporting
        println!("{: >2}: {}", i, result.len());

        // start the next string
        let mut next = format!("{}", &result[0..=0]);

        // for each character until the end
        // push the middle char + end of the pair
        for i in 0..(result.len() - 1) {
            let next_char = mutations.get(&result[i..i + 2])?;
            next.push(*next_char);
            next.push_str(&result[i+1..i+2]);
        }
        result = next;
    }

    println!("{: >2}: {}", steps, result.len());

    //reduce the string making a hashmap of the char => count
    let char_occurences = result.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    Some(char_occurences.values().max()? - char_occurences.values().min()?)
}

fn better(p: &Path, steps: usize) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;
    let (template, raw_dict) = parse_input(&raw_input)?;

    // Let's map CH -> B into something more useful, like CH -> [CB, BH]
    //  - CH -> B, is making CBH, which is a occurrence of CB and BH
    let net_effect_mutations: HashMap<&str, Vec<String>> = raw_dict
        .iter()
        .map(|(&k, v)| {
            (
                k,
                vec![format!("{}{}", &k[0..=0], v), format!("{}{}", v, &k[1..=1])],
            )
        })
        .collect();

    // Let's split the template into pairs, with a count of 1
    // NNCB -> [NN => 1, NC => 1, NB => 1]
    let initial_pairs = (0..(template.len() - 1)).fold(HashMap::new(), |mut acc, i| {
        let pair = &template[i..=(i + 1)];
        *acc.entry(pair).or_insert(0) += 1;
        acc
    });

    // for each step:
    //   loop over each pair_count,
    //   get the output mutations with the current count of those pairs
    //   reduce those down again to get the new counts
    // Note: Some items in the dict may produce the same next pairs.
    //   e.g. NH -> C, NN -> C both produce NC, thus why we reduce
    let pair_counts = (0..steps).fold(initial_pairs, |acc, i| {
        println!("{: >2}: {}", i, acc.len());
        acc
            .iter()
            .flat_map(|(k, v)| {
                let mut new_pairs = net_effect_mutations.get(*k).unwrap().iter();
                vec![
                    (new_pairs.next().unwrap(), v),
                    (new_pairs.next().unwrap(), v),
                ]
            })
            .fold(HashMap::new(), |mut acc, (pair, count)| {
                *acc.entry(pair).or_insert(0) += count;
                acc
            })
    });

    // For each pair, we can count the first char, since the second char is going to appear in
    // another pair
    // Except, of course the last character, that's only ever counted once
    let mut char_occurences = pair_counts
        .iter()
        .map(|(k, v)| (k.chars().next().unwrap(), v))
        .fold(HashMap::new(), |mut acc, (c, v)| {
            *acc.entry(c).or_insert(0) += v;
            acc
        });
    *char_occurences.get_mut(&template.chars().last()?)? += 1;

    Some(char_occurences.values().max()? - char_occurences.values().min()?)
}


fn parse_input(input: &str) -> Option<(&str, HashMap<&str, char>)> {
    let mut parts = input.split("\n\n");

    let template = parts.next()?;

    let instructions = parts
        .next()?
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let mut parts = x.split(" -> ");
            (parts.next().unwrap(), parts.next().unwrap().chars().next().unwrap())
        })
        .collect::<HashMap<&str, char>>();

    Some((template, instructions))
}
