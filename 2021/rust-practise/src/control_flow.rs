pub fn run() {
    conditional();
    loops();
}
    
fn conditional() {
    let age = 35;

    // Note, no warning on unused branch etc.
    if age >= 40 {
        println!("massive");
    } else if age >= 35 {
        println!("sort of massive");
    } else {
        println!("not massive");
    }

    // no () needed, && = and, || = or, no ternary
    // if something { true } else { false }; <-- budget ternary (why bother though)
}

fn loops() {

    // infinite loop
    let mut i = 0;
    loop {
        i += 1; //no i++;
        
        if i >= 10 {
            break;
        }
    }
    println!("{}", i); //no unused warning if i wasn't printed 


    let mut j = 0;
    while j <= 100 {
        if j % 15 == 0 {
            println!("FizzBuzz");
        } else if j % 5 == 0 {
            println!("Buzz");
        } else if j % 3 == 0 {
            println!("Fizz");
        } else {
            if j == 2 { 
                j += 1;
                continue; //don't print because we can
            }
            println!("{}", j);
        }

        j += 1
    }

    for k in 0..10 { //doesn't run for 10, but yes for 9
        println!("{}", k * -1)
    }
}
