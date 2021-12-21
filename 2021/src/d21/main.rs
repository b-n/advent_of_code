use std::collections::HashMap;

pub fn run() {
    //println!("Part 1: {}", p01(4, 8).unwrap());
    println!("Part 1: {}", p01(7, 3).unwrap());
    //println!("Part 2: {}", p02(4, 8).unwrap());
    println!("Part 2: {}", p02(7, 3).unwrap());
}

struct Die {
    value: usize,
    pub rolls: usize,
}

impl Die {
    fn new() -> Self {
        Die {
            value: 99,
            rolls: 0,
        }
    }
}

impl Iterator for Die {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.rolls += 1;
        self.value = (self.value + 1) % 100;
        Some(self.value)
    }
}

fn p01(p1_start: usize, p2_start: usize) -> Option<usize> {
    let mut players = vec![(0, p1_start - 1), (0, p2_start - 1)];

    let mut die = Die::new();
    let mut turn = 0;
    while players[0].0 < 1000 && players[1].0 < 1000 {
        let current_player = turn % 2;
        let to_move = die.next()? + die.next()? + die.next()? + 3;

        players[current_player].1 = (players[current_player].1 + to_move) % 10;
        players[current_player].0 += players[current_player].1 + 1;

        turn += 1;
    }

    let mut loser = players[0].0;
    if loser >= 1000 {
        loser = players[1].0;
    }

    Some(die.rolls * loser)
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Universe {
    players: Vec<(usize, usize)>,
}

fn p02(p1_start: usize, p2_start: usize) -> Option<usize> {
    let possible_movements: HashMap<usize, usize> = (1..=3)
        .flat_map(|i| {
            (1..=3)
                .flat_map(|j| (1..=3).map(|k| i + j + k).collect::<Vec<usize>>())
                .collect::<Vec<usize>>()
        })
        .fold(HashMap::new(), |mut acc, i| {
            *acc.entry(i).or_insert(0) += 1;
            acc
        });

    let start_players = vec![
        (p1_start - 1, 0),
        (p2_start - 1, 0)
    ];

    let mut wins: HashMap<usize, usize> = HashMap::new();

    let mut universum: HashMap<Universe, usize> = HashMap::new();
    universum.insert(Universe { players: start_players }, 1);

    let mut turn = 0;
    while universum.len() > 0 {
        let player = turn % 2;
        let mut next_universes = HashMap::new();

        for (universe, i) in universum.iter() {
            for (m, j) in possible_movements.iter() {
                let mut players = universe.players.clone();

                let new_pos = (players[player].0 + m) % 10;
                let new_score = players[player].1 + new_pos + 1;

                if new_score >= 21 {
                    *wins.entry(player).or_insert(0) += i * j;
                } else {
                    players[player] = (new_pos, new_score);
                    *next_universes.entry(Universe { players }).or_insert(0) += i * j;
                }
            }
        }
        universum = next_universes;
        turn += 1;
    }

    let p1_wins = *wins.get(&0)?;
    let p2_wins = *wins.get(&1)?;
    if p1_wins > p2_wins {
        Some(p1_wins)
    } else {
        Some(p2_wins)
    }
}
