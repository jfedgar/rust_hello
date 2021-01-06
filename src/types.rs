/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char) - single quotes
Tuples
Arrays (fixed length)

see strings.rs for strings
*/

// Rust is a statically typed language, which means that it must know the types of all
//   variables at compile time, however, the compiler can usually infer what type we
//   want to use based on the value and how we use it.

pub fn run() {
    //default is i32
    let x = 1;

    // default float is f64
    let y = 2.5;

    // explicitly define
    let z: i64 = 455555;

    let is_greater = 10 > 5;

    // chars, can by any unicode
    let a1 = 'f';
    let face = '\u{1f600}';

    // get max i32 size
    println!("max i32: {}", std::i32::MAX);
}
