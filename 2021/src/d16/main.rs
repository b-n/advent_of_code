use std::fs;
use std::path::Path;

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

    fn value(&self) -> u64 {
        match self.t {
            0 => self.children.iter().map(|x| x.value()).sum::<u64>(),
            4 => self.value,
            _ => 0,
        }
    }
}

fn p01(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;

    let mut bit_stack: Vec<u8> = raw_input
        .chars()
        .filter(|x| x != &'\n')
        .map(|x| u64::from_str_radix(&format!("{}", x), 16).unwrap() as u8)
        .collect();

    let (_, operator) = get_packet(&mut bit_stack)?;

    Some(operator.get_version() as usize)
}

fn p02(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;

    let mut bit_stack: Vec<u8> = raw_input
        .chars()
        .filter(|x| x != &'\n')
        .map(|x| u64::from_str_radix(&format!("{}", x), 16).unwrap() as u8)
        .collect();

    let (_, operator) = get_packet(&mut bit_stack)?;

    Some(operator.value() as usize)
}

fn get_packet(bit_stack: &mut Vec<u8>) -> Option<(u64, Packet)> {
    let version = get_bits(3, bit_stack);
    let t = get_bits(3, bit_stack);

    if t == 4 {
        let (used_bits, literal) = get_literal(version, t, bit_stack)?;
        Some((used_bits, literal))
    } else {
        let (used_bits, operator) = get_operator(version, t, bit_stack)?;
        Some((used_bits, operator))
    }
}

fn get_literal(version: u64, t: u64, bit_stack: &mut Vec<u8>) -> Option<(u64, Packet)> {
    let mut value = 0;
    let mut used_bits = 6;
    loop {
        let num = get_bits(5, bit_stack);
        value <<= 4;
        value |= num & 15;
        //numbers.push((num & 15) as u8);
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

fn get_operator(version: u64, t: u64, bit_stack: &mut Vec<u8>) -> Option<(u64, Packet)> {
    let length_type = get_bits(1, bit_stack);
    let mut children = vec![];
    let mut used_bits = 7;
    match length_type {
        0 => {
            let mut total_length = get_bits(15, bit_stack);
            used_bits += 15;
            used_bits += total_length;
            while total_length > 0 {
                let (used, child) = get_packet(bit_stack)?;
                total_length -= used;
                children.push(child);
            }
        }
        1 => {
            let sub_packet_count = get_bits(11, bit_stack);
            used_bits += 11;

            for _ in 0..sub_packet_count {
                let (used, child) = get_packet(bit_stack)?;
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

fn get_bits(size: usize, hexes: &mut Vec<u8>) -> u64 {
    let mut output: u64 = 0;
    let mut to_get = size.clone();

    while to_get > 0 {
        if hexes.len() == 0 {
            to_get = 0;
            continue;
        }

        if to_get >= 4 {
            output = (output << 4) | (hexes.remove(0) as u64);
            to_get -= 4;
            continue;
        }
        output = (output << to_get) | (hexes[0] >> (4 - to_get)) as u64;

        for i in 0..hexes.len() {
            hexes[i] = (hexes[i] << to_get) & 15;
            if i < hexes.len() - 1 {
                hexes[i] |= hexes[i + 1] >> (4 - to_get);
            }
        }

        if hexes[hexes.len() - 1] == 0 {
            hexes.remove(hexes.len() - 1);
        }

        to_get = 0;
    }
    output
}
