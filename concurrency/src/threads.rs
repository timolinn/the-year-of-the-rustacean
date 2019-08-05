
#[allow(unused_imports)]
#[allow(unused_variables)]

use std::{thread,io};
use std::time::Duration;

pub fn main() {
    let oceans = vec!["pacific", "atlantic", "indian", "arctic"];
    let handle = thread::spawn(move || {
        println!("{:?}", oceans);
        for i in 1..10 {
            println!("Hi this is number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi this is number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // println!("Enter data");
    // io::stdin().read_line(&mut String::new()).expect("You must enter data");
    handle.join().unwrap();
}
