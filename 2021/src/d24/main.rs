use std::fs;
use std::path::Path;
use std::collections::HashMap;

pub fn run() {
    let path = Path::new("./input/24");

    println!("Part 1: {}", p01(&path).unwrap());
    println!("Part 2: {}", p02(&path).unwrap());
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

type HeapPos = usize;
type Instruction = (Op, HeapPos, Option<HeapPos>, Option<i64>);
type Heap = Vec<i64>;
const HEAP_SIZE: usize = 4;

fn rhs(instruction: &Instruction, heap: &Heap) -> Option<i64> {
    match instruction {
        (_, _, Some(v), None) => Some(heap[*v]),
        (_, _, None, Some(v)) => Some(*v),
        _ => unreachable!()
    }
}

type InstructionError = ();
fn process(instruction: &Instruction, heap: &mut Heap) -> Option<InstructionError> {
    let (op, var, _, _) = instruction;
    let r = rhs(instruction, heap)?;
    let l = heap[*var];

    match op {
        Op::Add => heap[*var] += r,
        Op::Mul => heap[*var] *= r,
        Op::Div => {
            if r == 0 { return Some(()) }
            heap[*var] /= r;
        },
        Op::Mod => {
            if l == 0 || r <= 0 { return Some(()) }
            heap[*var] %= r;
        },
        Op::Eql => {
            heap[*var] = if l == r {
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
    let full_stack = generate_stack(&raw_input)?;

    let mut stacks = vec![];
    let mut next_stack = vec![];
    for i in full_stack.iter() {
        if i.0 == Op::Inp {
            if next_stack.len() > 0 {
                stacks.push(next_stack);
            }
            next_stack = vec![];
        }
        next_stack.push(i.clone());
    }
    stacks.push(next_stack);

    let (_, max) = alu_extent(&stacks)?;

    Some(max)
}

fn p02(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;
    let full_stack = generate_stack(&raw_input)?;

    let mut stacks = vec![];
    let mut next_stack = vec![];
    for i in full_stack.iter() {
        if i.0 == Op::Inp {
            if next_stack.len() > 0 {
                stacks.push(next_stack);
            }
            next_stack = vec![];
        }
        next_stack.push(i.clone());
    }
    stacks.push(next_stack);

    let (min, _) = alu_extent(&stacks)?;

    Some(min)
}

fn alu_extent(stacks: &Vec<Vec<&Instruction>>) -> Option<(usize, usize)> {
    let start_heap = vec![0; HEAP_SIZE];
    let mut heap_extent: HashMap<Heap, ((usize, usize), i64)> = HashMap::new();
    heap_extent.insert(start_heap, ((0, 0), 0));

    let mut i = 0;
    for stack in stacks {
        let mut next_heaps: HashMap<Heap, ((usize, usize), i64)> = HashMap::new();

        for (heap, ((min, max), _)) in heap_extent.iter() {
            for i in 1..=9 {
                let mut heap = heap.clone();

                for ins in stack.iter() {
                    match ins.0 {
                        Op::Inp => {
                            heap[ins.1] = i as i64;
                            None
                        },
                        _ => {
                            process(&ins, &mut heap)
                        }
                    };
                }

                let z = heap[3];

                let heap_extent = next_heaps.entry(heap).or_insert(((usize::MAX, 0), z));
                let next_min = usize::min(heap_extent.0.0, min * 10 + i);
                let next_max = usize::max(heap_extent.0.1, max * 10 + i);
                *heap_extent = ((next_min, next_max), z);
            }
        }
        i += 1;
        heap_extent.clear();
        heap_extent = next_heaps;
    }

    let z0s = heap_extent.values()
        .filter(|(_, z)| z == &0)
        .map(|(extent, _)| extent);

    let z_min = *z0s.clone().map(|(v, _)| v).min()?;
    let z_max = *z0s.map(|(_, v)| v).max()?;

    Some((z_min, z_max))
}

fn generate_stack(input: &str) -> Option<Vec<Instruction>> {
    let instructions = input.split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let mut parts = x.split(" ");
            let op = parts.next().unwrap();
            let var = char_as_pos(parts.next().unwrap().chars().next().unwrap());
            let val = match parts.next() {
                Some(v) => { 
                    match v.parse::<i64>() {
                        Ok(v) => (None, Some(v)),
                        Err(_) => (Some(char_as_pos(v.chars().next().unwrap())), None)
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

fn char_as_pos(c: char) -> HeapPos {
    match c {
        'w' => 0,
        'x' => 1,
        'y' => 2,
        'z' => 3,
        _ => unreachable!()
    }
}
