
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Alive {
    name: String,
}

struct NotAlive {
    name: String,
}

enum Matter {
    Living(Alive),
    NonLiving(NotAlive)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {

    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 0);
    let lb = IpAddr::V6(String::from("::1"));
    println!("Hello, world!");
}
