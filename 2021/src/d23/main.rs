use std::collections::HashMap;

pub fn run() {
    println!("Part 1: {}", p01("ADCABCBD").unwrap());
    //println!("Part 2: {}", p02(path).unwrap());
}

#[derive(Debug)]
struct Node {
    id: usize,
    cost_1_dests: Vec<usize>,
    cost_2_dests: Vec<usize>,
    has: Option<char>,
    wants: Option<char>,
}

fn graph() -> HashMap<usize, Node> {
    let mut res = HashMap::new();
    res.insert(0, Node { id: 0, cost_1_dests: vec![4], cost_2_dests: vec![], has: None, wants: Some('A') });
    res.insert(1, Node { id: 1, cost_1_dests: vec![5], cost_2_dests: vec![], has: None, wants: Some('B') });
    res.insert(2, Node { id: 2, cost_1_dests: vec![6], cost_2_dests: vec![], has: None, wants: Some('C') });
    res.insert(3, Node { id: 3, cost_1_dests: vec![7], cost_2_dests: vec![], has: None, wants: Some('D') });
    res.insert(4, Node { id: 4, cost_1_dests: vec![0], cost_2_dests: vec![9,10], has: None, wants: Some('A') });
    res.insert(5, Node { id: 5, cost_1_dests: vec![1], cost_2_dests: vec![10,11], has: None, wants: Some('B') });
    res.insert(6, Node { id: 6, cost_1_dests: vec![2], cost_2_dests: vec![11,12], has: None, wants: Some('C') });
    res.insert(7, Node { id: 7, cost_1_dests: vec![3], cost_2_dests: vec![12,13], has: None, wants: Some('D') });
    res.insert(8, Node { id: 8, cost_1_dests: vec![9], cost_2_dests: vec![], has: None, wants: None });
    res.insert(9, Node { id: 9, cost_1_dests: vec![8], cost_2_dests: vec![3,10], has: None, wants: None });
    res.insert(10, Node { id: 10, cost_1_dests: vec![], cost_2_dests: vec![4,5,9,11], has: None, wants: None });
    res.insert(11, Node { id: 11, cost_1_dests: vec![], cost_2_dests: vec![5,6,10,12], has: None, wants: None });
    res.insert(12, Node { id: 12, cost_1_dests: vec![], cost_2_dests: vec![6,7,11,13], has: None, wants: None });
    res.insert(13, Node { id: 13, cost_1_dests: vec![14], cost_2_dests: vec![7,12], has: None, wants: None });
    res.insert(14, Node { id: 14, cost_1_dests: vec![13], cost_2_dests: vec![], has: None, wants: None });
    res
}

fn p01(input: &str) -> Option<usize> {
    let mut g = graph();
    for (i, c) in input.chars().enumerate() {
        g.get_mut(&i)?.has = Some(c);
    }

    println!("{:?}", g);
    
    Some(0)
}

//fn p02(p: &Path) -> Option<i64> {
    //let raw_input = fs::read_to_string(p).ok()?;

    //Some(0)
//}
