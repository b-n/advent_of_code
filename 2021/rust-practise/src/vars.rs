pub fn run() {
    //let = vars that can change
    let some_var = "something";
    let some_int = 32;

    //age = 38; <-- immutable
    
    let mut some_mutable = 33;
    println!("{}", some_mutable); //just to use the var

    some_mutable = 33; //mutable

    println!("{} {} {}", some_var, some_int, some_mutable);

    // const should be uppercase, literal consts (compile time)
    const ID: i32 = 001; // i32 = int32 required for const

    println!("{}", ID);

    let (_name, _age) = ("Ben", 35); //object expansion
}
