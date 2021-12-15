use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn run() {
    let path = Path::new("./input/15");

    println!("Part 1: {}", p01(path).unwrap());
    println!("Part 2: {}", p02(path).unwrap());
}

const CARDINAL_DIRS: &'static [(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

type Point = (usize, usize);
type Cost = usize;

fn p01(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;

    let grid = parse_input(&raw_input)?;

    let grid_max_y = grid.keys().map(|(y, _)| y).max()?;
    let grid_max_x = grid.keys().map(|(_, x)| x).max()?;

    let in_bounds = |(y, x): &Point| *x <= *grid_max_x && *y <= *grid_max_y;

    let mut point_cost: HashMap<Point, Cost> = HashMap::new();
    point_cost.insert((0, 0), 0);

    let mut next: HashMap<Point, Cost> = next_points(&(0, 0), 0, &grid, in_bounds, &mut point_cost);

    println!("{}[2J", 27 as char);
    while next.len() > 0 {
        let mut sorted_points: Vec<(Point, Cost)> =
            next.iter().map(|(p, cost)| (*p, *cost)).collect();
        sorted_points.sort_by(|a, b| a.1.cmp(&b.1));

        let (point, cost) = sorted_points.first()?;
        next.remove(point);

        print_pos(point, &point_cost);

        for (point, cost) in next_points(point, *cost, &grid, in_bounds, &mut point_cost) {
            point_cost.insert(point, cost);

            if !next.contains_key(&point) || next.get(&point)? > &cost {
                next.insert(point, cost);
            }
        }
    }

    let start = &(0, 0);
    let end = &(*grid_max_y, *grid_max_x);

    Some(*point_cost.get(end)? - *point_cost.get(start)?)
}

fn p02(p: &Path) -> Option<usize> {
    let raw_input = fs::read_to_string(p).ok()?;

    let raw_grid = parse_input(&raw_input)?;
    let height = raw_grid.keys().map(|(y, _)| y).max()? + 1;
    let width = raw_grid.keys().map(|(_, x)| x).max()? + 1;

    let mut grid: HashMap<Point, Cost> = HashMap::new();

    for i in 0..5 {
        for j in 0..5 {
            for ((y, x), cost) in raw_grid.iter() {
                let new_y = y + i * height;
                let new_x = x + j * width;
                let mut cost = cost + i + j;
                if cost > 9 {
                    cost -= 9
                }
                grid.insert((new_y, new_x), cost);
            }
        }
    }

    let grid_max_y = grid.keys().map(|(y, _)| y).max()?;
    let grid_max_x = grid.keys().map(|(_, x)| x).max()?;

    let in_bounds = |(y, x): &Point| *x <= *grid_max_x && *y <= *grid_max_y;

    let mut point_cost: HashMap<Point, Cost> = HashMap::new();
    point_cost.insert((0, 0), 0);

    let mut next: HashMap<Point, Cost> = next_points(&(0, 0), 0, &grid, in_bounds, &mut point_cost);
    
    //let f_factor: usize = 10;
    let f_score = |p: &Point, cost: Cost| {
        // A failed attempt at euclidean distance
        //let y = grid_max_y - p.0;
        //let x = grid_max_x - p.1;
        //((y * y * 1000 + x * x * 1000) as f64).sqrt() as usize + cost * f_factor
        cost
    };

    println!("{}[2J", 27 as char);
    let end = &(*grid_max_y, *grid_max_x);
    while next.len() > 0 {
        if point_cost.contains_key(end) {
            break;
        }
        let mut sorted_points: Vec<(Point, Cost, Cost)> =
            next.iter().map(|(p, cost)| {
                (*p, *cost, f_score(p, *cost))
            }).collect();
        sorted_points.sort_by(|a, b| a.2.cmp(&b.2));

        let (point, cost, _) = sorted_points.first()?;
        next.remove(point);

        print_pos(point, &point_cost);

        for (point, cost) in next_points(point, *cost, &grid, in_bounds, &mut point_cost) {
            point_cost.insert(point, cost);

            if !next.contains_key(&point) || next.get(&point)? > &cost {
                next.insert(point, cost);
            }
        }
    }

    let start = &(0, 0);

    Some(*point_cost.get(end)? - *point_cost.get(start)?)
}

fn print_pos(p: &Point, costs: &HashMap<Point, Cost>) {
    print!(
        "{}[2j{}[{};{}H{}",
        27 as char,
        27 as char,
        p.0 / 2,
        p.1,
        display_char(p, costs)
    );
}

fn display_char((y, x): &Point, costs: &HashMap<Point, Cost>) -> char {
    match if y % 2 == 0 {
        2 & (costs.contains_key(&(y + 1, *x)) as usize & 1)
    } else {
        1 & (costs.contains_key(&(y - 1, *x)) as usize & 2)
    } {
        1 => '▀',
        2 => '▄',
        _ => '█',
    }
}

fn next_points<F>(
    from: &Point,
    cost: Cost,
    chart: &HashMap<Point, Cost>,
    in_bounds: F,
    point_costs: &mut HashMap<Point, Cost>,
) -> HashMap<Point, Cost>
where
    F: FnMut(&Point) -> bool + Copy,
{
    let (y, x) = *from;
    // lets get all points that are in bounds
    // we have our current cost (e.g. the cost of the current point)
    // we know how much the next point will cost (and we want to return it
    // we shouldn't return a point if we know it's going to cost more than we currently have for
    // that point, that's just dumb

    let is_cheaper = |(p, cost): &(Point, Cost)| {
        !point_costs.contains_key(p) || point_costs.get(p).unwrap() > cost
    };

    CARDINAL_DIRS
        .iter()
        .map(|(c_y, c_x)| (((y as i32 + c_y) as usize), ((x as i32 + c_x) as usize)))
        .filter(in_bounds)
        .map(|p| (p, cost + chart.get(&p).unwrap()))
        .filter(is_cheaper)
        .collect()
}

fn parse_input(input: &str) -> Option<HashMap<Point, Cost>> {
    Some(
        input
            .split("\n")
            .enumerate()
            .filter(|(_, x)| !x.is_empty())
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| ((y, x), (c as usize) - ('0' as usize)))
                    .collect::<Vec<(Point, Cost)>>()
            })
            .collect(),
    )
}
