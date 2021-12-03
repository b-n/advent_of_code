use std::path::Path;
use crate::utils::file;
use std::io::{Lines, BufReader};

pub fn run() {
    let path = Path::new("./input/03");

    println!("Part 1: {}", p01(&path).unwrap());
}

fn p01(p: &Path) -> Option<u32> {
    let lines = file::read_to_lines(p);
    let agg = fill_2d_vector_from_lines(lines)?;

    let bin_length = agg.len();
    let total_rows = agg[0].len();

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    let u0: u32 = 0; // really rust?
    for (i, a) in agg.iter().enumerate() {
        let pos = bin_length - i - 1;

        let mut count0 = 0;
        for val in a.iter() {
            if *val == u0 { count0 += 1 }
        }

        println!("{} {}", count0, total_rows);
        
        if count0 > total_rows / 2  {
            gamma = gamma | (1 << pos);
        } else {
            epsilon = epsilon | (1 << pos);
        }
    }

    Some(gamma * epsilon)
}

//fn p02(p: &Path) -> Option<i32> {
    //let lines = file::read_to_lines(p);
    //let agg = fill_2d_vector_from_lines(lines)?;

    //let gamma = 0;
    //let epsilon = 0;

    //Some(gamma * epsilon)
//}

fn fill_2d_vector_from_lines(lines: Lines<BufReader<std::fs::File>>) -> Option<Vec<Vec<u32>>> {
    let mut agg: Vec<Vec<u32>> = vec![];

    for (i, line) in lines.enumerate() {
        let str_value = file::line_as_str(line);

        // initialise aggregate vector
        if i == 0 {
            for _ in 0..str_value.len() {
                agg.push(vec![]);
            }
        }
         
        for (j, c) in str_value.chars().enumerate() {
            agg[j].push(char_to_bin(c)?);
        }
    }

    Some(agg)
}

fn char_to_bin(c: char) -> Option<u32> {
    const RADIX: u32 = 2;
    c.to_digit(RADIX)
}
