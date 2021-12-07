use std::path::Path;
use crate::utils::file;

pub fn run() {
    let path = Path::new("./input/06");

    println!("Part 1: {}", p01(path).unwrap());
    println!("Part 2: {}", p02(path).unwrap());
}

#[derive(Clone, Debug)]
struct Spawner {
    pub day: usize,
    pub fish: usize,
    onhold: usize,
}

impl Spawner {
    fn new(day: usize) -> Self {
        Self { day, fish: 0, onhold: 0 }
    }

    // returns number of new fish
    fn tick(&mut self) -> Option<usize> {
        if self.day == 0 {
            self.day = 6;
            let new_fish = self.fish;

            self.fish += self.onhold;
            self.onhold = 0;
            Some(new_fish)
        } else {
            self.day -= 1;
            Some(0)
        }
    }

    fn add_fish(&mut self, to_add: usize) { self.onhold += to_add; }
}

#[allow(dead_code)]
fn print_fish(s: &Vec<Spawner>) {
    println!("{:?}", s.iter().map(|f| (f.day, f.fish)).collect::<Vec<(usize, usize)>>());
}

fn iterate_spawners(spawners: &mut Vec<Spawner>, days: usize) {
    for d in 0..days {
        let additions = spawners.iter_mut()
            .map(|s| s.tick().unwrap())
            .max().unwrap();
        spawners[(d + 2) % 7].add_fish(additions);
    }
}

fn p01(p: &Path) -> Option<usize> {
    let raw_input = file::line_as_str(file::read_to_lines(p).next()?)?;
    
    let mut spawners = (0..7)
        .map(|d| Spawner::new(d))
        .collect::<Vec<Spawner>>();

    for f in file::csv_to_vec::<usize>(raw_input).ok()?.iter() {
        spawners[*f].fish += 1;
    }

    iterate_spawners(&mut spawners, 80);

    //print_fish(&spawners);
    Some(spawners.iter().map(|s| s.fish + s.onhold).sum())
}

fn p02(p: &Path) -> Option<usize> {
    let raw_input = file::line_as_str(file::read_to_lines(p).next()?)?;
    
    let mut spawners = (0..7)
        .map(|d| Spawner::new(d))
        .collect::<Vec<Spawner>>();

    for f in file::csv_to_vec::<usize>(raw_input).ok()?.iter() {
        spawners[*f].fish += 1;
    }

    iterate_spawners(&mut spawners, 256);

    //print_fish(&spawners);
    Some(spawners.iter().map(|s| s.fish + s.onhold).sum())
}
