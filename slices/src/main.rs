
fn slice(word: &str) -> &str {
    let slice = &word[..4];
    println!("{}", slice);

    slice
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert string into array of bytes

    for (i, &item) in bytes.iter().enumerate() { // the enumerate returns a tuple of the index and element in the array
        if item == 32 { // i inspected the array of bytes, found space to be represented by 32
            println!("Found a space in index {}", i);
        }
        if item == b' ' { // b' ' = byte literal
            println!("This is also checks for space. it's called byte literal");
            return i;
        }
        println!("{} = {}", i, item);
    }
    println!("{:?}", bytes);
    s.len() // this code will not run if the return statement inside the forloop is executed first
}

fn main() {
    slice("Timothy");

    let name = String::from("Johnny Depp");
    let result = first_word(&name);
    println!("{}", result);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}", hello)
}
