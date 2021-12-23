use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn run() {
    println!("Part 1: {}", p01("ADCABCBD").unwrap());
    //println!("Part 2: {}", p02(path).unwrap());
}

#[derive(Debug)]
struct Node {
    to: Vec<usize>,
    allows_stopping: bool,
}

fn graph() -> HashMap<usize, Node> {
    let mut res = HashMap::new();
    res.insert( 0, Node { to: vec![4], allows_stopping: true, },);
    res.insert( 1, Node { to: vec![5], allows_stopping: true, },);
    res.insert( 2, Node { to: vec![6], allows_stopping: true, },);
    res.insert( 3, Node { to: vec![7], allows_stopping: true, },);
    res.insert( 4, Node { to: vec![0, 10], allows_stopping: true, },);
    res.insert( 5, Node { to: vec![1, 12], allows_stopping: true, },);
    res.insert( 6, Node { to: vec![2, 14], allows_stopping: true, },);
    res.insert( 7, Node { to: vec![3, 16], allows_stopping: true, },);
    res.insert( 8, Node { to: vec![9], allows_stopping: true, },);
    res.insert( 9, Node { to: vec![8, 10], allows_stopping: true, },);
    res.insert( 10, Node { to: vec![4, 9, 11], allows_stopping: false, },);
    res.insert( 11, Node { to: vec![10, 12], allows_stopping: true, },);
    res.insert( 12, Node { to: vec![5, 11, 13], allows_stopping: false, },);
    res.insert( 13, Node { to: vec![12, 14], allows_stopping: true, },);
    res.insert( 14, Node { to: vec![6, 13, 15], allows_stopping: false, },);
    res.insert( 15, Node { to: vec![14, 16], allows_stopping: true, },);
    res.insert( 16, Node { to: vec![7, 15, 17], allows_stopping: false, },);
    res.insert( 17, Node { to: vec![16, 18], allows_stopping: true, },);
    res.insert( 18, Node { to: vec![17], allows_stopping: true, },);
    res
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct NodePosition {
    points: Vec<char>,
}

impl NodePosition {
    fn new(points: &mut Vec<char>, len: usize) -> Self {
        points.resize_with(len, || '\0');
        Self {
            points: points.clone(),
        }
    }

    fn iter(&self) -> impl Iterator<Item = &char> + '_ {
        self.points.iter()
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
    ) -> Option<Vec<(NodePosition, usize)>> {
        let mut res = vec![];
        let visited: HashSet<usize> = self
            .position
            .iter()
            .enumerate()
            .filter(|(_, &c)| c != '\0')
            .map(|(i, _)| i)
            .collect();

        for (pos, c) in self.position.iter().enumerate() {
            if c == &'\0' {
                continue;
            }

            let mut visited2 = visited.clone();
            if pos > 7 {
                for i in 8..graph.len() {
                    visited2.insert(i);
                }
            }

            for (n_pos, steps) in traverse_graph(0, pos, graph, &mut visited2)?.iter() {
                if n_pos != &pos {
                    let mut next = self.position.points.clone();
                    next[*n_pos] = *c;
                    next[pos] = '\0';
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
    steps: usize,
    start: usize,
    graph: &HashMap<usize, Node>,
    visited: &mut HashSet<usize>,
) -> Option<HashMap<usize, usize>> {
    let node = graph.get(&start)?;

    let mut res = HashMap::new();
    if node.allows_stopping {
        res.insert(start, steps);
    }

    for pos in node.to.iter() {
        if visited.contains(pos) {
            continue;
        } else {
            res.extend(traverse_graph(steps + 1, *pos, graph, visited)?);
        }
    }

    Some(res)
}

fn dykastra(
    graph: &HashMap<usize, Node>,
    char_costs: &HashMap<char, usize>,
    start: &NodePosition,
    end: &NodePosition,
) -> Option<usize> {
    let mut position_costs: HashMap<NodePosition, usize> = HashMap::new();

    let mut search_items: BinaryHeap<NodeState> = BinaryHeap::new();
    search_items.push(NodeState {
        position: start.clone(),
        cost: 0,
    });

    let i = 0;
    while let Some(item) = search_items.pop() {
        if search_items.len() % 10000 == 0 {
            println!("{}", search_items.len());
        }
        if &item.position == end {
            return Some(item.cost);
        }

        for (position, cost) in item.next(graph, char_costs)? {
            if !position_costs.contains_key(&position) || &cost < position_costs.get(&position)? {
                position_costs.insert(position.clone(), cost);
                search_items.push(NodeState { position, cost })
            }
        }
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
    start.reserve(g.len() - start.len());

    let begin = NodePosition::new(&mut start, g.len());
    let end = NodePosition::new(&mut vec!['A', 'B', 'C', 'D', 'A', 'B', 'C', 'D'], g.len());

    let res = Some(dykastra(&g, &char_costs, &begin, &end));
    println!("{:?}", res);

    Some(0)
}

//fn p02(p: &Path) -> Option<i64> {
//let raw_input = fs::read_to_string(p).ok()?;

//Some(0)
//}
