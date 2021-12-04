use crate::utils::file;
use std::path::Path;

pub fn run() {
    let path = Path::new("./input/04");

    println!("Part 1: {}", p01(&path).unwrap());
    //println!("Part 2: {}", p02(&path).unwrap());
}

type BingoBoard = Vec<Vec<usize>>;

#[derive(Debug)]
struct BingoGame {
    numbers: Vec<usize>,
    players: Vec<BingoPlayer>,
}

#[derive(Debug)]
struct BingoPlayer {
    board: BingoBoard,
}

impl BingoPlayer {
    fn new(board: BingoBoard) -> Self {
        Self { board }
    }

    fn take(&mut self, num: usize) -> Option<bool> {
        let mut pos: (usize, usize) = (25,24);
        let mut halting = false;

        //find the value, if we find it, mark it done (100 = out of bounds)
        for (y, row) in self.board.iter_mut().enumerate() {
            for (x, val) in row.iter_mut().enumerate() {
                if *val == num {
                    pos = (y,x);
                    *val = 100;
                    halting = true;
                }
            }
            if halting { break; }
        }

        //self.print_board();

        // if we halted, we can check the finished row/col for a bingo
        let mut won = false;
        if halting {
            let mut col = true;
            let mut row = true;
            for i in 0..5 {
                row &= self.board[pos.0][i] == 100;
                col &= self.board[i][pos.1] == 100;
            }
            won = col || row;
        }

        Some(won)
    }

    fn get_board_sum(&self) -> Option<usize> {
        Some(self.board.iter()
            .flatten()
            .filter(|&x| *x < 100)
            .sum::<usize>())
    }

    #[allow(dead_code)]
    fn print_board(&self) {
        println!("Board for {}", self.get_board_sum().unwrap());
        for row in self.board.iter() {
            println!("{:?}", row);
        }
    }
}

fn p01(p: &Path) -> Option<usize> {
    let mut game = bingo_game_from_path(p)?;

    for num in game.numbers.iter() {

        for player in game.players.iter_mut() {
            if player.take(*num)? {
                return Some(num * player.get_board_sum()?);
            }
        }
    }
    Some(0)
}

fn bingo_game_from_path(p: &Path) -> Option<BingoGame> {
    let mut lines = file::read_to_lines(p);

    let raw_numbers = file::line_as_str(lines.next()?);

    let numbers: Vec<usize> = csv_to_vec(raw_numbers)?;
    let mut players: Vec<BingoPlayer> = vec![];

    // let's assume bingo boards start with an empty line
    while let Some(_) = lines.next() {
        let mut board = vec![];
        for _ in 0..5 {
            let str_value = file::line_as_str(lines.next()?);
            board.push(bingo_line_to_vec(str_value)?);
        }
        players.push(BingoPlayer::new(board));
    }

    Some(BingoGame { numbers, players })
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
