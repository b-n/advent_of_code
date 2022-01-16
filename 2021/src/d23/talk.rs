use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn run() {
    println!("{}", p01("ADCABCBD").unwrap());
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

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Board {
    nodes: Vec<char>,
}

impl Board {
    fn new(points: &mut Vec<char>, len: usize) -> Self {
        points.resize_with(len, || ' ');
        Self {
            nodes: points.clone(),
        }
    }

    fn iter(&self) -> impl Iterator<Item = &char> + '_ {
        self.nodes.iter()
    }

    fn print(&self, bottom_size: usize) {
        println!("#############");
        print!("#");
        for i in bottom_size..bottom_size+11 {
            print!("{}", self.nodes[i]);
        }
        println!("#");

        for i in (0..bottom_size/4).rev() {
            let pad = if i == (bottom_size/4) - 1 {
                '#'
            } else {
                ' '
            };
            println!("{pad}{pad}#{}#{}#{}#{}#{pad}{pad}", self.nodes[i * 4 + 0],self.nodes[i * 4 + 1],self.nodes[i * 4 + 2],self.nodes[i * 4 + 3], pad = pad);
        }
        println!("  #########\n");
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct BoardState {
    board: Board,
    cost: usize,
}

impl BoardState {
    fn next(
        &self,
        graph: &HashMap<usize, Node>,
        char_costs: &HashMap<char, usize>,
        bottom_size: usize,
    ) -> Option<Vec<(Board, usize)>> {
        let mut res = vec![];

        // Used later in the graph traversal - just needed a reuseable default
        let visited: HashSet<usize> = self
            .board
            .iter()
            .enumerate()
            .filter(|(_, &c)| c != ' ')
            .map(|(i, _)| i)
            .collect();
        

        // for each point (e.g. letter) on our board, let's see if we can find a new home
        let filled_points = self.board.iter().enumerate().filter(|(_, &c)| c != ' ');
        for (pos, c) in filled_points {
            if pos < bottom_size - 4 && self.board.nodes[pos + 4] != ' ' {
                continue;
            }

            // which column does this point want?
            let wants = match c {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                'D' => 3,
                _ => unreachable!()
            };


            // don't midify 'done' columns. This checks:
            // - is it already in the bottom, and in the correct column
            // - are all other values in this column also correct
            if pos < bottom_size && pos % 4 == wants {
                let mut completed = true;
                for i in 0..bottom_size/4 {
                    completed &= self.board.nodes[i * 4 + wants] == *c;
                }
                if completed {
                    continue;
                }
            }

            // get all possible locations for all points
            for (n_pos, steps) in traverse_graph(0, pos, graph, &mut visited.clone())?.iter() {
                // Check if we actually traversed the graph, and then filter some points
                if n_pos != &pos {
                    // We were in the top section, and stayed in the top section
                    if pos > bottom_size && *n_pos > bottom_size {
                        continue;
                    }
                    // We were in the bottom section, and stayed there
                    if pos < bottom_size && *n_pos < bottom_size {
                        continue;
                    }

                    // We went to the bottom (from the top), but
                    if *n_pos < bottom_size {

                        // wasn't the correct column
                        if n_pos % 4 != wants {
                            continue;
                        }
                        let mut all_same = true;
                        for i in 0..bottom_size/4 {
                            let c2 = self.board.nodes[i * 4 + wants];
                            all_same &= c2 ==*c || c2 == ' ';
                        }

                        // or they weren't all the same
                        if !all_same { continue; }
                    }

                    // We had a valid position!
                    // - Clone the board
                    // - Move the item
                    // - Push it to the return pile with it's cost
                    let mut next = self.board.nodes.clone();
                    next[*n_pos] = *c;
                    next[pos] = ' ';
                    res.push((
                        Board { nodes: next },
                        self.cost + char_costs.get(c)? * steps,
                    ));
                }
            }
        }
        Some(res)
    }
}

impl Ord for BoardState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for BoardState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn traverse_graph(
    steps: usize,
    pos: usize,
    graph: &HashMap<usize, Node>,
    visited: &mut HashSet<usize>,
) -> Option<HashMap<usize, usize>> {
    // Let's walk the graph
    // - We only need to visit each node once

    visited.insert(pos);
    let node = graph.get(&pos)?;

    // if we're allowed to stop here, add the node to the return list
    let mut res = HashMap::new();
    if node.allows_stopping {
        res.insert(pos, steps);
    }

    // keep walking for all connected nodes in the graph (except one's we've seen)
    for n_pos in node.to.iter() {
        if visited.contains(n_pos) {
            continue;
        }

        res.extend(traverse_graph(steps + 1, *n_pos, graph, visited)?);
    }

    Some(res)
}

fn dykastra(
    graph: &HashMap<usize, Node>,
    char_costs: &HashMap<char, usize>,
    bottom_size: usize,
    start: &Board,
    end: &Board,
) -> Option<usize> {
    let mut board_costs: HashMap<Board, (usize, Board)> = HashMap::new();

    let mut search_items: BinaryHeap<BoardState> = BinaryHeap::new();
    search_items.push(BoardState {
        board: start.clone(),
        cost: 0,
    });

    let mut i = 0;
    while let Some(item) = search_items.pop() {
        //Some reporting
        if i % 10000 == 0 {
            println!("Iteration: {}", i);
        }

        // Stopping check
        if &item.board == end {
            print_path(&board_costs, &item.board, bottom_size, start);
            return Some(item.cost);
        }

        // For each next position, add to search if:
        // - we don't have a value already
        //   OR
        // - we do have a value already, BUT it's cheaper
        for (board, cost) in item.next(graph, char_costs, bottom_size)? {
            if !board_costs.contains_key(&board) || cost < board_costs.get(&board)?.0 {
                board_costs.insert(board.clone(), (cost, item.board.clone()));
                search_items.push(BoardState { board, cost })
            }
        }
        
        i += 1;
    }

    None
}

fn print_path(history: &HashMap<Board, (usize, Board)>, item: &Board, bottom_size: usize, start: &Board) {
    let mut i = 0;
    let mut next = item;
    while let Some(prev) = history.get(&next) {
        println!("Permutation {} - Cost {}", i, prev.0);
        next.print(bottom_size);
        if &prev.1 == start {
            break;
        }
        next = &prev.1;
        i += 1;
    }

    println!("Permutation {} - Cost {}", i + 1, 0);
    start.print(bottom_size);
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

    let begin = Board::new(&mut start, g.len());
    let end = Board::new(&mut vec!['A', 'B', 'C', 'D', 'A', 'B', 'C', 'D'], g.len());
   
    println!("Searching paths");
    dykastra(&g, &char_costs, bottom_size, &begin, &end)
}
