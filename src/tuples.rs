// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("foo", "yoo", 37);

    println!("what {}, other {}, num {}", person.0, person.1, person.2);
}
