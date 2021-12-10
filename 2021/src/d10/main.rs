use crate::utils::file;
use std::collections::HashMap;
use std::path::Path;

pub fn run() {
    let path = Path::new("./input/10");

    println!("Part 1: {}", p01(path).unwrap());
    println!("Part 2: {}", p02(path).unwrap());
}

fn p01(p: &Path) -> Option<usize> {
    let lines = file::read_to_lines(p);

    Some(
        lines
            .map(|l| file::line_as_str(l).unwrap())
            .map(|l| check_line(l, true).unwrap())
            .sum(),
    )
}

fn p02(p: &Path) -> Option<usize> {
    let lines = file::read_to_lines(p);

    let mut scores = lines
        .map(|l| file::line_as_str(l).unwrap())
        .map(|l| check_line(l, false).unwrap())
        .filter(|v| v != &0)
        .collect::<Vec<usize>>();

    scores.sort();

    Some(scores[scores.len() / 2])
}

fn check_line(line: String, return_on_fail: bool) -> Option<usize> {
    let mut brackets = HashMap::new();
    brackets.insert('[', ']');
    brackets.insert('(', ')');
    brackets.insert('{', '}');
    brackets.insert('<', '>');

    let mut closing_stack: Vec<&char> = vec![];

    for c in line.chars() {
        if brackets.contains_key(&c) {
            closing_stack.push(brackets.get(&c)?);
        } else {
            let last_elem = closing_stack.len() - 1;
            if closing_stack[last_elem] == &c {
                closing_stack.remove(last_elem);
            } else {
                if return_on_fail {
                    return Some(match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => 0,
                    });
                } else {
                    closing_stack.clear();
                    break;
                }
            }
        }
    }

    if return_on_fail {
        return Some(0)
    }

    let bracket_value = |b: &char| -> usize {
        match b {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        }
    };

    Some(closing_stack.iter().rev().fold(0, |mut acc, b| {
        acc *= 5;
        acc += bracket_value(*b);
        acc
    }))
}
