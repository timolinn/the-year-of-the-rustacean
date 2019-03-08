#![allow(unused)]
#![allow(dead_code)]
use std::fs::File;
use std::io;
use std::io::Read;

fn result(p: &str) -> File {
    let f = File::open(p);

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => match File::create(p) {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
    f
}

fn propagate_error() -> Result<String, io::Error> {
     let f = File::open("jada.smith");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error)
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("error.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn open_and_unwrap() -> File {
    let f = File::open("hello.tx").unwrap();
    f
}

fn open_with_expect() -> File {
    let f = File::open("error.log").expect("File not found! 404!!!");
    f
}

fn main() {
    // let file = open_and_unwrap();
    // let file = result("hello.jsx");
    // let file = open_with_expect();
    fs::read_to_string("hello.txt");
    // panic!("crash and burn!!! 🔥🔥🔥🔥🎇🎇🎇");
    let f = File::open("app.log").map_err(|error| {
        if error.kind() == io::ErrorKind::NotFound {
            File::create("app.log").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem {:?}", error);
            })
        } else {
            panic!("Error opening file");
        }
    });
}
