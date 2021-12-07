use crate::utils::file;
use std::path::Path;

// Learnings:
// - I should use the auto formatter more often (thus the small changes in this commit)
// - Structs and functions aren't that bad really. Reminds me a lot of golang (what came first?)
// - Advent of code ramps up in the weekend (at least it's not during the week)
// - Iterators are "good". I should use them a lot more often in other languages. Efficient
//
// Rust Enjoyment factor: [+++/------] (+ 1/2, this problem makes it interesting I guess)

pub fn run() {
    let path = Path::new("./input/04");

    println!("Part 1: {}", p01(&path).unwrap());
    println!("Part 2: {}", p02(&path).unwrap());
}

type BingoBoard = Vec<Vec<usize>>;

#[derive(Debug)]
struct BingoGame {
    numbers: Vec<usize>,
    players: Vec<BingoPlayer>,
    playing: usize,
}

#[derive(Debug)]
struct BingoPlayer {
    won: bool,
    board: BingoBoard,
}

impl BingoPlayer {
    fn new(board: BingoBoard) -> Self {
        Self { won: false, board }
    }

    fn take(&mut self, num: usize) -> Option<bool> {
        let mut pos: (usize, usize) = (25, 24);
        let mut halting = false;

        //find the value, if we find it, mark it done (100 = out of bounds)
        for (y, row) in self.board.iter_mut().enumerate() {
            for (x, val) in row.iter_mut().enumerate() {
                if *val == num {
                    pos = (y, x);
                    *val = 100;
                    halting = true;
                }
            }
            if halting {
                break;
            }
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

        self.won = won;

        Some(won)
    }

    fn get_board_sum(&self) -> Option<usize> {
        Some(
            self.board
                .iter()
                .flatten()
                .filter(|&x| *x < 100)
                .sum::<usize>(),
        )
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

fn p02(p: &Path) -> Option<usize> {
    let mut game = bingo_game_from_path(p)?;

    let mut last_player = usize::MAX;
    for num in game.numbers.iter() {
        for player in game.players.iter_mut() {
            if !player.won && player.take(*num)? {
                game.playing -= 1
            }
        }

        // store the last player, but keep playing until they are finished
        if game.playing == 1 {
            last_player = game
                .players
                .iter()
                .enumerate()
                .filter(|(_, player)| !player.won)
                .map(|(index, _)| index)
                .collect::<Vec<usize>>()[0];
        }

        if game.playing == 0 {
            return Some(game.players[last_player].get_board_sum()? * num);
        }
    }

    Some(0)
}

fn bingo_game_from_path(p: &Path) -> Option<BingoGame> {
    let mut lines = file::read_to_lines(p);

    let raw_numbers = file::line_as_str(lines.next()?).ok()?;

    let numbers: Vec<usize> = csv_to_vec(raw_numbers)?;
    let mut players: Vec<BingoPlayer> = vec![];

    // let's assume bingo boards start with an empty line
    while let Some(_) = lines.next() {
        let mut board = vec![];
        for _ in 0..5 {
            let str_value = file::line_as_str(lines.next()?).ok()?;
            board.push(bingo_line_to_vec(str_value)?);
        }
        players.push(BingoPlayer::new(board));
    }

    Some(BingoGame {
        playing: players.len(),
        numbers,
        players,
    })
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
