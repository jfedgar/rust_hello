// Primitive str = Immutable fixed-length string somewhere in memory (what we have been using in println)
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data
pub fn run() {
    // primitive:
    let hello = "hello";

    // String, growable
    let hello = String::from("hello");

    //get length
    println!("string length: {}", hello.len());

    // mutable string
    let mut hello1 = String::from("hello ");

    hello1.push('w');
    hello1.push_str("orld");

    println!("{}", hello1);
    // empty
    println!("{}", hello1.is_empty());
    // contains
    println!("{}", hello1.contains("world"));
    // replace
    println!("{}", hello1.replace("world", "there"));

    for word in hello1.split_whitespace() {
        println!("{}", word);
    }

    // create string with specific capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertions
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}
