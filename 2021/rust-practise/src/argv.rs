use std::env;

// run using: cargo run some commands here

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let command = args[1].clone(); // copy, instead of getting a ref

    println!("{}", command); //some

    if command == "some" {
        println!("someeee")
    } else {
        println!("nah")
    }
}
