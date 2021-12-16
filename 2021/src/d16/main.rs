//use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn run() {
    let path = Path::new("./input/16");

    println!("Part 1: {}", p01(path).unwrap());
    //println!("Part 2: {}", p02(path).unwrap());
}

#[derive(Debug)]
struct Literal {
    version: u64,
    numbers: Vec<u8>
}

#[derive(Debug)]
struct Operator {
    version: u64,
    operators: Vec<Operator>,
    literals: Vec<Literal>,
}

impl Operator {
    fn get_versions(&self) -> u64 {
        let mut versions = self.version;
        for l in self.literals.iter() {
            versions += l.version;
        }
        for o in self.operators.iter() {
            versions += o.get_versions();
        }
        versions
    }
}

fn p01(p: &Path) -> Option<usize> {
    //let raw_input = fs::read_to_string(p).ok()?;
    //let raw_input = "8A004A801A8002F478";
    //let raw_input = "620080001611562C8802118E34";
    let raw_input = "C0015000016115A2E0802F182340";
    //let raw_input = "A0016C880162017C3686B18A3D4780";
    //let raw_input = "38006F45291200";
    //let raw_input = "D2FE28";

    let mut bit_stack: Vec<u8> = raw_input.chars()
        .filter(|x| x != &'\n')
        .map(|x| {
            u64::from_str_radix(&format!("{}", x), 16).unwrap() as u8
        })
        .collect();

    let mut literals: Vec<Literal> = vec![];
    let mut operators: Vec<Operator> = vec![];

    while !bit_stack.is_empty() {
        match get_packet(&mut bit_stack)? {
            (_, Some(l), None) => literals.push(l),
            (_, None, Some(o)) => operators.push(o),
            _ => (),
        }
    }

    let mut versions: u64 = 0;
    for l in literals.iter() {
        versions += l.version;
    }
    for o in operators.iter() {
        versions += o.get_versions();
    }

    Some(versions as usize)
}

fn get_packet(bit_stack: &mut Vec<u8>) -> Option<(u64, Option<Literal>, Option<Operator>)> {
    let version = get_bits(3, bit_stack);
    let t = get_bits(3, bit_stack);
    
    if t == 4 {
        let (used_bits, literal) = get_literal(version, bit_stack)?;
        Some((used_bits, Some(literal), None))
    } else {
        let (used_bits, operator) = get_operator(version, bit_stack)?;
        Some((used_bits, None, Some(operator)))
    }
}

fn get_literal(version: u64, bit_stack: &mut Vec<u8>) -> Option<(u64, Literal)> {
    println!("literal");
    let mut numbers = vec![];
    let mut used_bits = 6;
    loop {
        let num = get_bits(5, bit_stack);
        numbers.push((num & 15) as u8);
        used_bits += 5;
        if num & 16 != 16 {
            break
        }
    }
    Some((used_bits, Literal { version, numbers }))
}

fn get_operator(version: u64, bit_stack: &mut Vec<u8>) -> Option<(u64, Operator)> {
    println!("operator");
    let length_type = get_bits(1, bit_stack);
    let mut literals = vec![];
    let mut operators = vec![];
    let mut used_bits = 7;
    if length_type == 0 {
        let mut total_length = get_bits(15, bit_stack);
        used_bits += total_length;
        println!("length_type: 0, {:?}", total_length);
        while total_length > 0 {
            println!("yo: {}",  total_length);
            match get_packet(bit_stack)? {
                (i, Some(l), None) => { literals.push(l); total_length -= i as u64; },
                (i, None, Some(o)) => { operators.push(o); total_length -= i as u64; },
                _ => ()
            }
        } 
    } else {
        let sub_packet_count = get_bits(11, bit_stack);
        used_bits += 11;
        println!("length_type: 1, {:?}", sub_packet_count);

        for _ in 0..sub_packet_count {
            match get_packet(bit_stack)? {
                (i, Some(l), None) => { literals.push(l); used_bits += i as u64; },
                (i, None, Some(o)) => { operators.push(o); used_bits += i as u64; },
                _ => ()
            }
        }
    }
    
    Some((used_bits, Operator{ version, literals, operators }))
}

//fn p02(p: &Path) -> Option<usize> {
    //let raw_input = fs::read_to_string(p).ok()?;

    //Some(0)
//}


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
                hexes[i] |= hexes[i+1] >> (4 - to_get);
            }
        }

        if hexes[hexes.len() - 1] == 0 {
            hexes.remove(hexes.len() -1);
        }

        to_get = 0;
    }
    output
}
