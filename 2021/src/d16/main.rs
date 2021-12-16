use super::bit_stack::BitStack;
use std::fs;
use std::path::Path;

// Learnings
// - I do like messing with bitwise operators
// - Reading is hard. But apparently that's common
// - This felt a little like work, but it wasn't hard so to speak
// - I can make hacky bit stacks, and they work!
//
// Rust Enjoyment Factor [+++++-----] +1, I'm getting more used to it I guess

pub fn run() {
    let path = Path::new("./input/16");

    println!("Part 1: {}", p01(path).unwrap());
    println!("Part 2: {}", p02(path).unwrap());
}

#[derive(Debug)]
struct Packet {
    version: u64,
    t: u64,
    value: u64,
    children: Vec<Packet>,
}

impl Packet {
    fn get_version(&self) -> u64 {
        self.version + self.children.iter().map(|x| x.get_version()).sum::<u64>()
    }

    fn values<'a>(&'a self) -> impl Iterator<Item = u64> + 'a {
        self.children.iter().map(|x| x.value())
    }

    fn value(&self) -> u64 {
        match self.t {
            0 => self.values().sum::<u64>(),
            1 => self.values().product::<u64>(),
            2 => self.values().min().unwrap(),
            3 => self.values().max().unwrap(),
            4 => self.value,
            5 => if self.children[0].value() > self.children[1].value() { 1 } else { 0 }
            6 => if self.children[0].value() < self.children[1].value() { 1 } else { 0 }
            7 => if self.children[0].value() == self.children[1].value() { 1 } else { 0 }
            _ => 0,
        }
    }
}

fn p01(p: &Path) -> Option<u64> {
    let input = fs::read_to_string(p).ok()?;

    let mut stack = input.parse::<BitStack>().ok()?;

    let (_, operator) = get_packet(&mut stack)?;

    Some(operator.get_version())
}

fn p02(p: &Path) -> Option<u64> {
    let input = fs::read_to_string(p).ok()?;

    let mut stack = input.parse::<BitStack>().ok()?;

    let (_, operator) = get_packet(&mut stack)?;

    Some(operator.value())
}

fn get_packet(stack: &mut BitStack) -> Option<(u64, Packet)> {
    let version = stack.pop(3);
    let t = stack.pop(3);

    if t == 4 {
        let (used_bits, literal) = get_literal(version, t, stack)?;
        Some((used_bits, literal))
    } else {
        let (used_bits, operator) = get_operator(version, t, stack)?;
        Some((used_bits, operator))
    }
}

fn get_literal(version: u64, t: u64, stack: &mut BitStack) -> Option<(u64, Packet)> {
    let mut value = 0;
    let mut used_bits = 6;
    loop {
        let num = stack.pop(5);
        value <<= 4;
        value |= num & 15;
        used_bits += 5;
        if num & 16 != 16 {
            break;
        }
    }
    Some((
        used_bits,
        Packet {
            version,
            t,
            value,
            children: vec![],
        },
    ))
}

fn get_operator(version: u64, t: u64, stack: &mut BitStack) -> Option<(u64, Packet)> {
    let length_type = stack.pop(1);
    let mut children = vec![];
    let mut used_bits = 7;
    match length_type {
        0 => {
            let mut total_length = stack.pop(15);
            used_bits += 15;
            used_bits += total_length;
            while total_length > 0 {
                let (used, child) = get_packet(stack)?;
                total_length -= used;
                children.push(child);
            }
        }
        1 => {
            let sub_packet_count = stack.pop(11);
            used_bits += 11;

            for _ in 0..sub_packet_count {
                let (used, child) = get_packet(stack)?;
                used_bits += used;
                children.push(child);
            }
        }
        _ => (),
    }

    Some((
        used_bits,
        Packet {
            version,
            t,
            value: 0,
            children,
        },
    ))
}
