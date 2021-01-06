// cargo new hello
// or mkdir hello; cd hello; cargo init

mod arrays;
mod print;
mod strings;
mod tuples;
mod types;
mod vars;

fn main() {
    // run from other module
    print::run();
    // replace within string
    println!("number: {}, name: {}", 1, "Brad");
    // replace within string with positional arguments
    println!("number: {0}, name: {1}, number:{0}", 1, "Brad");
    // named traits

    println!(
        "number: {num}, name: {nam}, number:{num}",
        num = 1,
        nam = "Brad"
    );
    // placeholder traits:
    println!("binary: {:b} hex: {:x} octal: {:o}", 10, 10, 10);
    // placeholder for debug (print array here):
    println!("{:?}", (12, true, "hello"));

    vars::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
}

// raw create (don't use with cargo):
//   rustc main.rs
// run debug mode:
//   cargo run
// just build:
//   cargo build
// build for production:
//   cargo build --release
