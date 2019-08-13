#[allow(unused_imports)]
#[allow(unused_variables)]

use std::sync::{Mutex, Arc};
use std::thread;

pub fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for idx in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            println!("Thread {} spawned", idx);
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}