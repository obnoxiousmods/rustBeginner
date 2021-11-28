// Primitive str = immutable fixed length string somewhere in memory
// String = growable, heap-allocated data structure - use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    // Push char
    hello.push('W');

    // Push string
    hello.push_str("orld!");

    println!("{}", hello);
