pub fn run() {
    let input = ((117, -89), (164, -140));

    println!("Part 1: {}", p01(input).unwrap());
    //println!("Part 2: {}", p02(input).unwrap());
}

type Position = (i64, i64);


fn p01((top_left, bottom_right): (Position, Position)) -> Option<i64> {
    // we're finding highest y
    //  - This means we can ignore x
    //
    // we have a range -89 => -140
    // We know that Y needs to be negative at the end
    // We can work backwards from a minus number to the start, and find y = 0 (can we?)
    
   
    // starting at tl.y, if we have a value that's never reached 0, and is decreasing, then not this (it's not high
    // if we're reached_zero, let's start testing whether the end is in range

    
    let mut end_y_velo = -1;
    //'outer: loop {
    loop {
    //for _ in 0..100 {
        //println!("{}", end_y_velo);
        let mut y_velo = end_y_velo;
        let mut y_pos = top_left.1;
        //for _ in 0..100 {
        let mut i = 0;

        let mut last_pos = top_left.1;
        let mut reached_zero = false;
        let mut go = true;
        loop {
            y_pos -= y_velo;
            y_velo += 1;
            
            if !reached_zero && y_pos < last_pos {
                println!("{} We're already falling, bad, should be higher", end_y_velo);
                break;
            }
            last_pos = y_pos;

            if y_pos >= 0 {
                reached_zero = true;
            }

            if reached_zero {
                println!("{} zero check {}, {}, {}", end_y_velo, y_pos, y_velo, top_left.1 - bottom_right.1);
                go &= y_pos > 0 && y_pos < top_left.1 - bottom_right.1;
            }
            if !go {
                // if we're breaking here, that means we found a start pos
                break;
            }

            //println!("pos {}, velo {}", y_pos, y_velo);
            i += 1;
        }
        let start_y_velo = y_velo;
        
        //println!("{} {}", i, y_pos);
        if !go { break; }


        //if y_pos > 0
        //}
        end_y_velo -= 1;
    }


    Some(0)
}

//fn p01(top_left: (i64, i64), bottom_right: (i64, i64)) -> Option(i64) {
  //Some(0)
//}
