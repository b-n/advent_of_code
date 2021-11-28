
use std::mem; //shorten std::mem::fn syntax to mem::fn

pub fn run() {
    //primitives
    //ints, [u|i][8|16|32|64|128] e.g. u32 unsigned 32
    //floats, f[32|64]
    //boolean, bool
    //chars, char ''
    //strings, ""

    // rust will figure out most types. but:
    // x = 1 <-- i32
    // y = 2.1 <-- f64
    // let z: i64 = some_int; <-- force type
    // let is_true = true;
    // let a1 = ''; <-- char (unicode, not ascii) , not string \u{1F600} = smiley
    // std::i32::MAX <-- const for size of i32
    
    //strings();
    //tuples();
    //arrays();
    vectors();
}

fn strings() {
    let mut hello = String::from("Hello"); // mutable, stack allocated etc
    println!("{:?}", (&hello, hello.len(), hello.capacity())); // need reference since we're passing it to println! (borrowed ref)

    hello.push(' '); // yes char
    hello.push_str("some string"); // yes more string

    println!("{} {}", hello, hello.capacity());

    // handy string funcs:
    // is_empty()
    // contains()
    // replace() <-- returns new instance of string, doesn't mutate
    
    for word in hello.split_whitespace() {
        println!("{}", word)
    }

    assert_eq!(17, hello.len()); // throws excpetion if not equal
}

fn tuples() {
    //handy for returning multiple values from a fn
    let some_arr: (&str, i32) = ("Ben", 32);

    println!("{} {}", some_arr.0, some_arr.1);
    println!("{:?}", some_arr);
}

fn arrays() {
    // Notes:
    //  - Arrays have fixed length (good luck with push)
    let arr: [i32; 3] = [1, 2, 3]; //size needs to match number of values

    println!("{:?} {}", arr, arr[0]); // can't be {} because there's no std::display for arrays, {:?} does though 
    println!("Array bytes {}", mem::size_of_val(&arr));

    let slice: &[i32] = &arr[1..3]; //ignore first element. end is not included
    println!("{:?}", slice);
}

fn vectors() {
    // in memory arrays etc. resizable, etc etc.
    let mut vec: Vec<i32> = vec![1, 2, 3];

    println!("{:?} {}", vec, vec[0]);

    vec.push(5);
    println!("{:?}", vec);

    // Other fns:
    // - pop
    
    // basic loop of iterable
    let mut count = 0;
    for i in vec.iter() {
        count += i; // i = element as vector type, i32/ handy
    }
    println!("{}", count);

    // mutating array = need to be explicit
    for i in vec.iter_mut() {
        *i *= 2; //i = ref, so *i to get value and multiply it
    }
    println!("{:?}", vec);
}
