use std::fs::File;
use std::io::ErrorKind;

fn result(p: &str) -> File {
    let f = File::open(p);

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
    f
}

fn main() {
    // panic!("crash and burn!!! 🔥🔥🔥🔥🎇🎇🎇");
    let f = File::open("app.log").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("app.log").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem {:?}", error);
            })
        } else {
            panic!("Error opening file");
        }
    });
}
