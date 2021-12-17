use std::collections::HashSet;

pub fn run() {
    let input = ((117, -89), (164, -140));
    //let input = ((20, -5), (30, -10));

    println!("Part 1: {}", p01(input).unwrap());
    println!("Part 2: {}", p02(input).unwrap());
}

type Position = (i64, i64);

fn p01(input: (Position, Position)) -> Option<i64> {
    let (max_y, _) = get_y_values(input)?;

    Some(max_y)
}

fn p02((top_left, bottom_right): (Position, Position)) -> Option<i64> {
    let (_, y_values) = get_y_values((top_left, bottom_right))?;

    let mut min_x = 1;
    while (min_x * (min_x + 1)) / 2 < top_left.0 { 
        min_x += 1;
    } 
    let max_x = bottom_right.0;

    let mut total = 0;

    for y in y_values.iter() {

        for x in min_x..=max_x {
            let mut x_pos = 0;
            let mut y_pos = 0;
            let mut x_velo = x;
            let mut y_velo = *y;

            let found = loop {
                x_pos += x_velo;
                if x_velo > 0 {
                    x_velo -= 1;
                }
                y_pos += y_velo;
                y_velo -= 1;

                if x_pos > bottom_right.0 || y_pos < bottom_right.1 {
                    break false;
                }

                if x_pos >= top_left.0 && x_pos <= bottom_right.0 && y_pos <= top_left.1 && y_pos >= bottom_right.1 {
                    break true;
                }
            };

            if found {
                total += 1;
            }
        }
    }

    Some(total)
}

fn get_y_values((top_left, bottom_right): (Position, Position)) -> Option<(i64, HashSet<i64>)> {
    let mut begin_values = HashSet::new();

    let mut max_max_y = 0;
    for init_y in (bottom_right.1)..(-2 * bottom_right.1) {
        
        let mut y_velo = init_y;
        let mut y_pos = 0;
        let mut max_y = 0;
        let found = loop {
            y_pos += y_velo;
            y_velo -= 1;

            if y_pos > max_y {
                max_y = y_pos;
            }

            if y_pos < bottom_right.1 {
                break false;
            } 

            if y_pos <= top_left.1 {
                break true;
            }
        };

        if found {
            begin_values.insert(init_y);
            if max_y > max_max_y {
                max_max_y = max_y;
            }
        }
    }

    Some((max_max_y, begin_values))
}
