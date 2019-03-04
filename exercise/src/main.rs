#![allow(unused_variables)]

pub mod exercise;

fn main() {
    let numbers = vec![2, 4, 5, 6, 7];
    let sum = exercise::mean(&numbers);
    println!("{}", sum);
}
