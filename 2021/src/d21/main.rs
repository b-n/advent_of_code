use std::collections::{HashMap, HashSet};

pub fn run() {
    //println!("Part 1: {}", p01(7, 3).unwrap());
    //println!("Part 1: {}", p01(4, 8).unwrap());
    println!("Part 1: {}", p02(4, 8).unwrap());
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
    println!("{} {}", loser, die.rolls);

    Some(die.rolls * loser)
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Universe {
    p1_pos: usize,
    p1_points: usize,
    p2_pos: usize,
    p2_points: usize,
    turn: usize,
}

impl Universe {
    fn new(p1_pos: usize, p1_points: usize, p2_pos: usize, p2_points: usize, turn: usize) -> Self {
        Self {
            p1_pos, p1_points,
            p2_pos, p2_points,
            turn,
        }
    }
}

// a universe is a combination of positions and points (p1, points + p2)
// for a universe, we know the universes it creates
// given position

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

    println!("{:?}", possible_movements);

    let mut wins: HashMap<usize, usize> = HashMap::new();
        
    let mut universum: HashMap<Universe, usize> = HashMap::new();
    universum.insert(Universe::new(p1_start - 1, 0, p2_start - 1, 0, 0), 1);

    let mut to_check = HashSet::new();
    to_check.insert(Universe::new(p1_start - 1, 0, p2_start - 1, 0, 0));

    loop {
        let next_universe = to_check.iter().next();
        if next_universe == None {
            break;
        }
        let universe = next_universe.unwrap().clone();
        to_check.remove(&universe);
        println!("{}", to_check.len());

        let current_count = *universum.get(&universe)?;
        for (m, i) in possible_movements.iter() {
            if universe.turn == 0 {
                let new_pos = (universe.p1_pos + m) % 10;
                let new_score = universe.p1_points + new_pos + 1;

                if new_score >= 21 {
                    *wins.entry(0).or_insert(0) += current_count; 
                } else {
                    let next = Universe::new(new_pos, new_score, universe.p2_pos, universe.p2_points, 1);
                    *universum.entry(next).or_insert(0) += current_count * i;
                    to_check.insert(next.clone());
                }
            } else {
                let new_pos = (universe.p2_pos + m) % 10;
                let new_score = universe.p2_points + new_pos + 1;

                if new_score >= 21 {
                   *wins.entry(1).or_insert(0) += current_count; 
                } else {
                    let next = Universe::new(universe.p1_pos, universe.p1_points, new_pos, new_score, 0);
                    *universum.entry(next).or_insert(0) += current_count * i;
                    to_check.insert(next.clone());
                }
            }
        }
    }
    println!("{:?}", wins);

    Some(0)
}
