use std::fs;
use std::path::Path;
use std::collections::HashMap;

pub fn run() {
    let path = Path::new("./input/24");

    println!("Part 1: {}", p01(&path).unwrap());
    //println!("Part 2: {}", p02("BCDABCAD").unwrap());
}

#[derive(Debug, Eq, PartialEq)]
enum Op {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

type Var = char;
type Instruction = (Op, Var, Option<Var>, Option<i64>);
type Heap = HashMap<Var, i64>;

fn rhs(instruction: &Instruction, heap: &Heap) -> Option<i64> {
    match instruction {
        (_, _, Some(v), None) => Some(*heap.get(v)?),
        (_, _, None, Some(v)) => Some(*v),
        _ => unreachable!()
    }
}

type InstructionError = ();
fn process(instruction: &Instruction, heap: &mut Heap) -> Option<InstructionError> {
    println!("{:?}", instruction);
    let (op, var, _, _) = instruction;
    let r = rhs(instruction, heap)?;
    let l = heap.entry(*var).or_insert(0);

    match op {
        Op::Add => *l += r,
        Op::Mul => *l *= r,
        Op::Div => {
            if r == 0 { return Some(()) }
            *l /= r;
        },
        Op::Mod => {
            if *l == 0 || r <= 0 { return Some(()) }
            *l %= r;
        },
        Op::Eql => {
            *l = if *l == r {
                1
            } else {
                0
            };
        },
        _ => ()
    }
    None
}


fn p01(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;
    let stack = generate_stack(&raw_input)?;
    let mut stacks = vec![];
    let mut next_stack = vec![];
    for i in stack.iter() {
        if i.0 == Op::Inp {
            if next_stack.len() > 0 {
                stacks.push(next_stack);
            }
            next_stack = vec![];
        }
        next_stack.push(i.clone());
    }
    stacks.push(next_stack);

    for stack in stacks {
        println!("{:?}", stack);
        'int: for i in (1..=9).rev() {
            let mut heap: Heap = HashMap::new();
            for ins in stack.iter() {
                let res = match ins.0 {
                    Op::Inp => {
                        heap.insert(ins.1, i);
                        None
                    },
                    _ => {
                        process(&ins, &mut heap)
                    }
                };
                // None = no error
                // if we get anything, we break for this i
                match res {
                    Some(_) => {
                        println!("error on {}", i);
                        continue 'int;
                    }
                    None => (),
                }
            }
            println!("{} {}", i, heap.get(&'z')?);
            if heap.get(&'z')? == &0 {
                println!("{}", i);
                break;
            }
        }
    }
    Some(0)
}

fn generate_stack(input: &str) -> Option<Vec<Instruction>> {
    let instructions = input.split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let mut parts = x.split(" ");
            let op = parts.next().unwrap();
            let var = parts.next().unwrap().chars().next().unwrap();
            let val = match parts.next() {
                Some(v) => { 
                    match v.parse::<i64>() {
                        Ok(v) => (None, Some(v)),
                        Err(_) => (Some(v.chars().next().unwrap()), None)
                    }
                }
                _ => (None, None)
            };
            match op {
                "inp" => (Op::Inp, var, None, None),
                "add" => (Op::Add, var, val.0, val.1),
                "mul" => (Op::Mul, var, val.0, val.1),
                "div" => (Op::Div, var, val.0, val.1),
                "mod" => (Op::Mod, var, val.0, val.1),
                "eql" => (Op::Eql, var, val.0, val.1),
                _ => unreachable!(),
            }
        }).collect();

    Some(instructions)
}


//fn p02(input: &str) -> Option<usize> {
    //Some(0)
//}
