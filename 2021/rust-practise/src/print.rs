pub fn run() {
    println!("{name} is a {thing}", name = "you", thing = "cool");
    println!("{0:b} {0:x} {0:o}", 10);
    // {:?} = debug, handy
    println!("{:?}", (10, "hello"));
}
