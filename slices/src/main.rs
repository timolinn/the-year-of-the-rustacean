
fn slice(word: &str, num: usize) {
    let slice = &word[..num];
    println!("{}", slice);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // convert string into array of bytes

    for (i, &item) in bytes.iter().enumerate() { // the enumerate returns a tuple of the index and element in the array
        if item == 32 { // i inspected the array of bytes, found space to be represented by 32
            println!("Found a space in index {}", i);
        }
        if item == b' ' { // b' ' = byte literal
            println!("This is also checks for space. it's called byte literal");
            return &s[..i];
        }
        println!("{} = {}", i, item);
    }
    println!("{:?}", bytes);
    //s.len() // this code will not run if the return statement inside the for loop is executed first
    &s[..s.len()]
}

fn args() {
    for arg in std::env::args() {
        println!("{}", arg);
    }
}

fn main() {
    args();
    let me = String::from("Timothy");
    slice(&me, 4);

    let name = String::from("Johnny Depp");
    let result = first_word(&name);
    println!("{}", result);

    let s = String::from("hello world");
    let hello = &s[..5]; // starts from index 0
    let world = &s[6..11];

    println!("{}", hello)
}
