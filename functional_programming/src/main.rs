//! Functional programming features in Rust
//! Functional programming is a programming paradigm
//! like OOP whereby pure functions are composed
//! to form useful programs as to using objects
//! in OOP paradigm

//! Refer to closures folder for full code
fn main() {
    let print_hello = || {
        println!("Hello, world!");
    };

    print_hello();
}
