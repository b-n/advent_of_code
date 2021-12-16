use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn run() {
    let path = Path::new("./input/16_example");

    println!("Part 1: {}", p01(path).unwrap());
    //println!("Part 2: {}", p02(path).unwrap());
}

#[derive(Debug)]
struct Literal {
    version: u8,
    numbers: Vec<u8>
}

fn p01(p: &Path) -> Option<usize> {
    //let raw_input = fs::read_to_string(p).ok()?;
    //let raw_input = "8A004A801A8002F478";
    let raw_input = "D2FE28";

    let mut hexes: Vec<u8> = raw_input.chars()
        .map(|x| {
            u64::from_str_radix(&format!("{}", x), 16).unwrap() as u8
        })
        .collect();

    let mut literals: Vec<Literal> = vec![];

    while !hexes.is_empty() {
        let version = get_bits(3, &mut hexes);
        let t = get_bits(3, &mut hexes);

        println!("{:#b} {:#b}", version, t);

        if t == 4 {
            let mut numbers = vec![];
            let mut used_bits = 6;
            loop {
                let num = get_bits(5, &mut hexes);
                numbers.push((num & 15) as u8);
                used_bits += 5;
                if num & 16 != 16 {
                    break
                }
            }
            println!("{:?}", numbers);
            println!("draining {} {}", used_bits, used_bits % 4);
            if used_bits % 4 > 0 {
                get_bits(used_bits % 4, &mut hexes);
            }

            literals.push(Literal { version: version as u8, numbers })
        }
    }

    println!("{:?}", literals);
      

    //    100 = literal value
    // VVVTTT000000000000000
    //   !000 = operator value
    //
    //    110 = not 100, so operator
    // VVVTTTILLLLLLLLLLLLLLLLLLL<SUB PACKET>
    //       I = 1 or 0 (11 or 15 bit length L

    Some(0)
}

//fn p02(p: &Path) -> Option<usize> {
    //let raw_input = fs::read_to_string(p).ok()?;

    //Some(0)
//}


fn get_bits(size: usize, hexes: &mut Vec<u8>) -> u64 {
    let mut output: u64 = 0;
    let mut to_get = size.clone();

    //println!("get_bits {}", to_get);
    while to_get > 0 {
        //println!("{} {}", to_get, output);
        if to_get >= 4 {
            output = (output << 4) | (hexes.remove(0) as u64);
            to_get -= 4;
            continue;
        }
        //println!("{} {:#b}", output, hexes[0] >> 4 - to_get);
        output = (output << to_get) | (hexes[0] >> (4 - to_get)) as u64;

        for i in 0..hexes.len() {
            //println!("{:#b} {:#b} {:#b}", hexes[i], hexes[i] << to_get, (hexes[i] << to_get) & 7);

            hexes[i] = (hexes[i] << to_get) & 15;
            //println!("{:#b}", hexes[i]);
            if i < hexes.len() - 1 {
                hexes[i] |= hexes[i+1] >> (4 - to_get);
            }
            //println!("end shifting {:#b}", hexes[i]);
        }

        if hexes[hexes.len() - 1] == 0 {
            hexes.remove(hexes.len() -1);
        }

        to_get = 0;
    }
    //println!("{}", output);
    output
}
