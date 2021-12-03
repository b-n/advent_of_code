use std::path::Path;
use crate::utils::file;

pub fn run() {
    let path = Path::new("./input/03");

    println!("Part 1: {}", p01(&path).unwrap());
    println!("Part 2: {}", p02(&path).unwrap());
}

// Learnings:
// - I really should just read the problem, I started with a transposed 2d array since that is how
//   it looked like it wanted to be counted, but a 2dmatrix was fine
// - I like me some bitwise operations. epsilon is fun, xor 1's makes cool things happen
// - Rust is just c but with inferred types and garbage collection
// - Embrace Option and Result. Your life will be better (unwrap is for chumps)
// - Recursion is pretty cool - my first attempt was inefficient
// - .iter().enumerate() will be my friend
// - .collect() should be my friend, but I need to get used to reference notation
// 
// Rust enjoyment factor [+++-------]

fn p01(p: &Path) -> Option<usize> {
    let lines = file::read_to_lines(p);

    let vec = file::lines_as_vec2d(lines)?;

    let bin_length = vec[0].len();

    let mut bit_counts: Vec<usize> = vec![0; bin_length]; 

    for line in vec.iter() {
        for (x, bit) in line.iter().enumerate() {
            if *bit == '1' { bit_counts[x] += 1 }
        } 
    }

    let mut res: Vec<char> = vec![];
    for count in bit_counts.iter() {
        let mut bit = '0';
        if *count >= vec.len() / 2 {
            bit = '1'
        }
        res.push(bit);
    }

    let gamma: usize = binvec_as_usize(&res)?;
    // 001101 * 111111 = 110010 (flips the bits). can only do that so much though
    let epsilon: usize = gamma ^ (usize::pow(2, bin_length as u32) - 1);

    Some(gamma * epsilon)
}

fn p02(p: &Path) -> Option<usize> {
    let lines = file::read_to_lines(p);

    let vec = file::lines_as_vec2d(lines)?;

    // why vec.iter().collect() ? because I want the inner vector to be a reference since that is
    // what is being filtered down (no need to copy it, we want REFS!
    // .collect() will [always] make a new Vector though, so we that's how we get nice recursion on
    // a new vector made out of refs
    let oxygen: usize = binvec_as_usize(filter_vec_for_sig_bit(vec.iter().collect(), 0, true)?)?;
    let c02: usize = binvec_as_usize(filter_vec_for_sig_bit(vec.iter().collect(), 0, false)?)?;

    Some(oxygen * c02)
}

fn filter_vec_for_sig_bit(vec2d: Vec<&Vec<char>>, pos: usize, msb: bool) -> Option<&Vec<char>> {
    if vec2d.len() == 1 {
        return Some(vec2d[0]);
    }

    let mut bit_count = 0;
    for vec in vec2d.iter() {
        if vec[pos] == '1' {
            bit_count += 1;
        }
    }

    // this logic looks hard, but it's not. Think about it
    let mut sb = '0';
    if msb && bit_count >= (vec2d.len() + 1) / 2 {
        sb = '1';
    }
    if !msb && bit_count < (vec2d.len() + 1) / 2 {
        sb = '1'
    }

    let next_vectors: Vec<&Vec<char>> = vec2d.iter()
        .filter(|v| v[pos] == sb)
        .map(|v| *v)
        .collect();

    filter_vec_for_sig_bit(next_vectors, pos + 1, msb)
}

fn binvec_as_usize(vec: &Vec<char>) -> Option<usize> {
    let mut res: usize = 0;
    let vec_len = vec.len();
    for (i, a) in vec.iter().enumerate() {
        if *a == '1' {
            res |= 1 << (vec_len - i - 1);
        }
    } 
    Some(res)
}
