#![allow(unused)]

extern crate regex;

use std::io;
use regex::Regex;

fn main() {
    let mut word = String::new();
    let text = io::stdin().read_line(&mut word).expect("Failed to read line");
    let re = Regex::new(word.trim()).unwrap();
    let quote = "Every face, every shop, bedroom window, public-house, and
      dark square is a picture feverishly turned--in search of what?
      It is the same with books. What do we seek through millions of pages?
      picture every where!!!";

    for line in quote.lines() {
        match re.find(line) {
          Some(_) => println!("{}", line),
          None => ()
        }
    }
}
