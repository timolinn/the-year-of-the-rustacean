#[allow(unused_imports)]
#[allow(unused_variables)]

use std::sync::mpsc;// mpsc multiple producer, single consumer
use std::thread;
use std::time::Duration;

pub fn main() {
//     let (tx, rx): (mpsc::Sender<u64>, mpsc::Receiver<u64>) = mpsc::channel(); // Returns the Sender and Receiver halves

    // thread::spawn(move || {
    //     let value = 50;
    //     tx.send(value).unwrap();
    // });
//     drop(tx);
//     let recvd = rx.recv().unwrap();
//     println!("Got: {}", recvd);

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        for num in vec!["more", "messages", "for", "her"] {
            tx.send(num.to_string()).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}