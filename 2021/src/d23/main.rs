use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn run() {
    //println!("Part 1: {}", p01("BCDABCAD").unwrap());
    println!("Part 2: {}", p02("BCDABCAD").unwrap());
}

#[derive(Debug)]
struct Node {
    to: Vec<usize>,
    allows_stopping: bool,
}

fn graph() -> HashMap<usize, Node> {
    let mut res = HashMap::new();
    res.insert(0, Node { to: vec![4], allows_stopping: true });
    res.insert(1, Node { to: vec![5], allows_stopping: true });
    res.insert(2, Node { to: vec![6], allows_stopping: true });
    res.insert(3, Node { to: vec![7], allows_stopping: true });
    res.insert(4, Node { to: vec![0,10], allows_stopping: true });
    res.insert(5, Node { to: vec![1,12], allows_stopping: true });
    res.insert(6, Node { to: vec![2,14], allows_stopping: true });
    res.insert(7, Node { to: vec![3,16], allows_stopping: true });
    res.insert(8, Node { to: vec![9], allows_stopping: true });
    res.insert(9, Node { to: vec![8,10], allows_stopping: true });
    res.insert(10, Node { to: vec![4,9,11], allows_stopping: false });
    res.insert(11, Node { to: vec![10,12], allows_stopping: true });
    res.insert(12, Node { to: vec![5,11,13], allows_stopping: false });
    res.insert(13, Node { to: vec![12,14], allows_stopping: true });
    res.insert(14, Node { to: vec![6,13,15], allows_stopping: false });
    res.insert(15, Node { to: vec![14,16], allows_stopping: true });
    res.insert(16, Node { to: vec![7,15,17], allows_stopping: false });
    res.insert(17, Node { to: vec![16,18], allows_stopping: true });
    res.insert(18, Node { to: vec![17], allows_stopping: true });
    res
}

fn graph2() -> HashMap<usize, Node> {
    let mut res = HashMap::new();
    res.insert(0, Node { to: vec![4], allows_stopping: true });
    res.insert(1, Node { to: vec![5], allows_stopping: true });
    res.insert(2, Node { to: vec![6], allows_stopping: true });
    res.insert(3, Node { to: vec![7], allows_stopping: true });
    res.insert(4, Node { to: vec![0,8], allows_stopping: true });
    res.insert(5, Node { to: vec![1,9], allows_stopping: true });
    res.insert(6, Node { to: vec![2,10], allows_stopping: true });
    res.insert(7, Node { to: vec![3,11], allows_stopping: true });
    res.insert(8, Node { to: vec![4,12], allows_stopping: true });
    res.insert(9, Node { to: vec![5,13], allows_stopping: true });
    res.insert(10, Node { to: vec![6,14], allows_stopping: true });
    res.insert(11, Node { to: vec![7,15], allows_stopping: true });
    res.insert(12, Node { to: vec![8,16], allows_stopping: true });
    res.insert(13, Node { to: vec![9,17], allows_stopping: true });
    res.insert(14, Node { to: vec![10,18], allows_stopping: true });
    res.insert(15, Node { to: vec![11,19], allows_stopping: true });
    res.insert(16, Node { to: vec![17], allows_stopping: true });
    res.insert(17, Node { to: vec![16,18], allows_stopping: true });
    res.insert(18, Node { to: vec![14,17,19], allows_stopping: false });
    res.insert(19, Node { to: vec![18,20], allows_stopping: true });
    res.insert(20, Node { to: vec![15,19,21], allows_stopping: false });
    res.insert(21, Node { to: vec![20,22], allows_stopping: true });
    res.insert(22, Node { to: vec![16,21,23], allows_stopping: false });
    res.insert(23, Node { to: vec![22,24], allows_stopping: true });
    res.insert(24, Node { to: vec![17,23,25], allows_stopping: false });
    res.insert(25, Node { to: vec![24,26], allows_stopping: true });
    res.insert(26, Node { to: vec![25], allows_stopping: true });
    res
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct NodePosition {
    points: Vec<char>,
}

impl NodePosition {
    fn new(points: &mut Vec<char>, len: usize) -> Self {
        points.resize_with(len, || ' ');
        Self {
            points: points.clone(),
        }
    }

    fn iter(&self) -> impl Iterator<Item = &char> + '_ {
        self.points.iter()
    }

    fn print(&self, bottom_size: usize) {
        print!("#");
        for i in bottom_size..bottom_size+11 {
            print!("{}", self.points[i]);
        }
        println!("#");

        for i in (0..bottom_size/4).rev() {
            println!("  #{}#{}#{}#{}#",self.points[i * 4 + 0],self.points[i * 4 + 1],self.points[i * 4 + 2],self.points[i * 4 + 3]);
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct NodeState {
    position: NodePosition,
    cost: usize,
}

impl NodeState {
    fn next(
        &self,
        graph: &HashMap<usize, Node>,
        char_costs: &HashMap<char, usize>,
        bottom_size: usize,
    ) -> Option<Vec<(NodePosition, usize)>> {
        let mut res = vec![];
        let visited: HashSet<usize> = self
            .position
            .iter()
            .enumerate()
            .filter(|(_, &c)| c != ' ')
            .map(|(i, _)| i)
            .collect();

        for (pos, c) in self.position.iter().enumerate() {
            if c == &' ' {
                continue;
            }

            let wants = match c {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                'D' => 3,
                _ => unreachable!()
            };

            let descending = pos > bottom_size;

            for (n_pos, steps) in traverse_graph(pos, descending, 0, pos, graph, &mut visited.clone(), bottom_size)?.iter() {
                if n_pos != &pos {
                    if *n_pos < bottom_size {
                        if *n_pos >= 4 && (self.position.points[n_pos - 4] == ' ' || self.position.points[n_pos - 4] != *c) {
                            continue;
                        }
                        if *n_pos < 4 && n_pos % 4 != wants {
                            continue;
                        }
                    }
                    let mut next = self.position.points.clone();
                    next[*n_pos] = *c;
                    next[pos] = ' ';
                    res.push((
                        NodePosition { points: next },
                        self.cost + char_costs.get(c)? * steps,
                    ));
                }
            }
        }
        Some(res)
    }
}

impl Ord for NodeState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for NodeState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn traverse_graph(
    start: usize,
    descending: bool,
    steps: usize,
    pos: usize,
    graph: &HashMap<usize, Node>,
    visited: &mut HashSet<usize>,
    bottom_size: usize
) -> Option<HashMap<usize, usize>> {
    visited.insert(pos);
    let node = graph.get(&pos)?;

    let mut res = HashMap::new();
    if node.allows_stopping && ((start < bottom_size && pos >= bottom_size) || descending) {
        res.insert(pos, steps);
    }

    for n_pos in node.to.iter() {
        if visited.contains(n_pos) {
            continue;
        }
        let desc = descending || (pos >= bottom_size && *n_pos < bottom_size);

        if pos >= bottom_size && *n_pos < bottom_size {
            res.extend(traverse_graph(start, desc, steps + 1, *n_pos, graph, visited, bottom_size)?);
        } else { 
            res.extend(traverse_graph(start, desc, steps + 1, *n_pos, graph, visited, bottom_size)?);
        }
    }

    Some(res)
}

fn dykastra(
    graph: &HashMap<usize, Node>,
    char_costs: &HashMap<char, usize>,
    bottom_size: usize,
    start: &NodePosition,
    end: &NodePosition,
) -> Option<usize> {
    let mut position_costs: HashMap<NodePosition, usize> = HashMap::new();

    let mut search_items: BinaryHeap<NodeState> = BinaryHeap::new();
    search_items.push(NodeState {
        position: start.clone(),
        cost: 0,
    });

    let mut i = 0;
    while let Some(item) = search_items.pop() {
        if item.cost > 10000 {
            println!("{}", item.cost);
            item.position.print(bottom_size);
        }

        if &item.position == end {
            return Some(item.cost);
        }

        for (position, cost) in item.next(graph, char_costs, bottom_size)? {
            if !position_costs.contains_key(&position) || &cost < position_costs.get(&position)? {
                position_costs.insert(position.clone(), cost);
                search_items.push(NodeState { position, cost })
            }
        }
        
        i += 1;
    }

    None
}

fn p01(input: &str) -> Option<usize> {
    let g = graph();

    let mut char_costs = HashMap::new();
    char_costs.insert('A', 1);
    char_costs.insert('B', 10);
    char_costs.insert('C', 100);
    char_costs.insert('D', 1000);

    let mut start = input.chars().collect::<Vec<char>>();
    let bottom_size = start.len();

    let begin = NodePosition::new(&mut start, g.len());
    let end = NodePosition::new(&mut vec!['A', 'B', 'C', 'D', 'A', 'B', 'C', 'D'], g.len());
    
    //let begin = NodePosition::new(&mut vec![' ', 'B', 'C', 'D', ' ', ' ', ' ', ' ', 'D', 'A', ' ', 'C'], g.len());
    //begin.print();
    //let mut visited: HashSet<usize> = begin
            //.iter()
            //.enumerate()
            //.filter(|(_, &c)| c != ' ')
            //.map(|(i, _)| i)
            //.collect();
    //println!("{:?}", visited);

    //let res = traverse_graph(9, 0, 0, 9, &g, &mut visited);

    let res = Some(dykastra(&g, &char_costs, bottom_size, &begin, &end));
    println!("{:?}", res);

    Some(0)
}

fn p02(input: &str) -> Option<usize> {
    let g = graph2();

    let mut char_costs = HashMap::new();
    char_costs.insert('A', 1);
    char_costs.insert('B', 10);
    char_costs.insert('C', 100);
    char_costs.insert('D', 1000);

    let first = &input[0..4];
    let middle = "DBACDCBA";
    let last = &input[4..8];
    let mut start = format!("{}{}{}", first, middle, last).chars().collect::<Vec<char>>();
    let bottom_size = start.len();

    let begin = NodePosition::new(&mut start, g.len());
    begin.print(bottom_size);
    let end = NodePosition::new(&mut vec!['A', 'B', 'C', 'D', 'A', 'B', 'C', 'D'], g.len());
    

    let res = Some(dykastra(&g, &char_costs, bottom_size, &begin, &end));
    println!("{:?}", res);

    Some(0)
}

