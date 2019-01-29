#![allow(unused_variables, unused_mut, non_snake_case)]
#[derive(Debug)]
struct Coord(i64, i64);

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: u32,
    sex: String,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle {
            width,
            height
        }
    }

    fn set_width(&mut self, new_width: u32) -> () {
        self.width = new_width;
    }

    fn set_height(&mut self, new_height: u32) -> () {
        self.height = new_height;
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn main() {

    let MrBean = User {
        email: String::from("johnenglish@mi7.com"),
        name: String::from("Johnny English"),
        age: 18,
        sex: String::from("Male"),
        active: true
    };

    let mut MrsBean = User {
        email: String::from("mrsjohnenglish@mi7.com"),
        name: String::from("Sandra English"),
        sex: String::from("Female"),
        ..MrBean
    };

    let naija = Coord(4, 14);
    println!("{:?}", naija);

    let mut rect = Rectangle::new(12, 4);
    println!("{}", rect.area());

    rect.set_width(40);
    rect.set_height(50);
    Rectangle::square(6);
    println!("{:#?}", rect);

    println!("The area of this rectangle is {}", area(&rect));
}
