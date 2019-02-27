#![allow(unused_variables)]
fn main() {
    let v: Vec<i32> = Vec::new();
    v.push(3);
    v.push(2);

    let n = vec![1, 2, 3];
    let one = &n[0];

    // using .get
    match n.get(2) {
        Some(three) => println!("matched letter {}", three),
        None => println!("invalid element")
    }
}
