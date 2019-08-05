#[allow(unused_imports)]
#[allow(unused_variables)]

use std::sync::Mutex;

pub fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 9;
    }

    println!("{:?}", m);
}