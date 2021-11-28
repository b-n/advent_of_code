pub fn run() {
    params("Hello", "Ben");

    println!("{}", add(1, 2));

    let n3 = 10;
    let adder = |n1: i32, n2: i32| n1 + n2 + n3; // closure with n3, can access blockscoped vars. ew, but okay?

    println!("{}", adder(1, 4));
}

fn params(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2 //no return! hi ruby! (without semi)
}
