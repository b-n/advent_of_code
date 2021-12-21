pub fn run() {
    println!("Part 1: {}", p01(7, 3).unwrap());
    //println!("Part 1: {}", p01(4, 8).unwrap());
}

struct Die {
    value: usize,
    pub rolls: usize
}

impl Die {
    fn new() -> Self {
        Die { value: 99, rolls: 0 }
    }
}

impl Iterator for Die {
    type Item = usize;
    fn next (&mut self) -> Option<Self::Item> {
        self.rolls += 1;
        self.value = (self.value + 1) % 100;
        Some(self.value)
    }
}


fn p01(p1_start: usize, p2_start: usize) -> Option<usize> {
    let mut players = vec![(0, p1_start - 1), (0, p2_start -1)];
    
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
