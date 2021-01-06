// variables are immutable by default
// rust is block-scoped

pub fn run() {
    let name = "foo";
    let mut age = "30";

    println!("My name is {}, age is {}", name, age);

    //const (must define type)
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple variables
    let (my_name, my_age) = ("foo", "30");

}