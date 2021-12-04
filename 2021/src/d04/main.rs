use crate::utils::file;
use std::path::Path;

pub fn run() {
    let path = Path::new("./input/04_example");

    println!("Part 1: {}", p01(&path).unwrap());
    //println!("Part 2: {}", p02(&path).unwrap());
}

type BingoBoard = Vec<Vec<usize>>;

#[derive(Debug)]
struct BingoGame {
    numbers: Vec<usize>,
    boards: Vec<BingoBoard>,
}

fn p01(p: &Path) -> Option<usize> {
    let game = bingo_game_from_path(p)?;

    Some(0)
}

fn bingo_game_from_path(p: &Path) -> Option<BingoGame> {
    let mut lines = file::read_to_lines(p);

    let raw_numbers = file::line_as_str(lines.next()?);

    let numbers: Vec<usize> = csv_to_vec(raw_numbers)?;
    let mut boards: Vec<BingoBoard> = vec![];

    // let's assume bingo boards start with an empty line
    while let Some(_) = lines.next() {
        let mut board = vec![];
        for _ in 0..5 {
            let str_value = file::line_as_str(lines.next()?);
            board.push(bingo_line_to_vec(str_value)?);
        }
        boards.push(board);
    }

    Some(BingoGame { numbers, boards })
}

fn bingo_line_to_vec(s: String) -> Option<Vec<usize>> {
    // thanks rust? split_ascii_whitespace ignores leading whitespace!
    Some(
        s.split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>(),
    )
}

fn csv_to_vec(s: String) -> Option<Vec<usize>> {
    Some(s.split(",").map(|x| x.parse::<usize>().unwrap()).collect())
}
