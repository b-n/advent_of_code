use std::path::Path;
use crate::utils::file;
use std::io::{Lines, BufReader};

pub fn run() {
    let path = Path::new("./input/03");

    println!("Part 1: {}", p01(&path).unwrap());
    println!("Part 2: {}", p02(&path).unwrap());
}

fn p01(p: &Path) -> Option<u32> {
    let lines = file::read_to_lines(p);
    let agg = fill_2d_vector_from_lines(lines)?;

    let bin_length = agg.len();
    let total_rows = agg[0].len();

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for (i, a) in agg.iter().enumerate() {
        let pos = bin_length - i - 1;

        let mut count0 = 0;
        for val in a.iter() {
            if *val == '0' { count0 += 1 }
        }

        if count0 > total_rows / 2  {
            gamma = gamma | (1 << pos);
        } else {
            epsilon = epsilon | (1 << pos);
        }
    }

    Some(gamma * epsilon)
}

fn p02(p: &Path) -> Option<u32> {
    let lines = file::read_to_lines(p);
    let agg = fill_2d_vector_from_lines(lines)?;

    let oxygen: u32 = value_at_position_in_vec(&agg, &filter_vec_for_sig_bit(&agg, true)?)?;
    let c02: u32 = value_at_position_in_vec(&agg, &filter_vec_for_sig_bit(&agg, false)?)?;

    Some(oxygen * c02)
}


fn filter_vec_for_sig_bit(vec: &Vec<Vec<char>>, msb: bool) -> Option<usize> {
    let mut res: usize = 0;

    let mut possible_index: Vec<usize> = (0..vec[0].len()).collect();

    for a in vec.iter() {
        let mut count1 = 0;

        for (j, val) in a.iter().enumerate() { 
            if possible_index.contains(&j) {
                if *val == '1' { count1 += 1 }
            }
        }

        let mut sb = '0';

        if msb && count1 >= (possible_index.len() + 1) / 2 {
            sb = '1';
        }
        if !msb && count1 < (possible_index.len() + 1) / 2 {
            sb = '1'
        }

        // keep values that we have in our array, and match the sb
        let mut to_keep: Vec<usize> = vec![];
        for (j, val) in a.iter().enumerate() {
            if possible_index.contains(&j) {
                if *val == sb { to_keep.push(j) }  
            }
        }

        possible_index.retain(|&x| to_keep.contains(&x));

        if possible_index.len() == 1 {
            res = possible_index[0];
            break;
        }
    }

    Some(res)
}

fn value_at_position_in_vec(vec: &Vec<Vec<char>>, position: &usize) -> Option<u32> {
    let mut res: u32 = 0;
    for (i, a) in vec.iter().enumerate() {
        if a[*position] == '1' {
            res |= 1 << (vec.len() - i - 1);
        }
    }
    Some(res)
}

fn fill_2d_vector_from_lines(lines: Lines<BufReader<std::fs::File>>) -> Option<Vec<Vec<char>>> {
    let mut agg: Vec<Vec<char>> = vec![];

    for (i, line) in lines.enumerate() {
        let str_value = file::line_as_str(line);

        // initialise aggregate vector
        if i == 0 {
            for _ in 0..str_value.len() {
                agg.push(vec![]);
            }
        }
         
        for (j, c) in str_value.chars().enumerate() {
            agg[j].push(c);
        }
    }

    Some(agg)
}
