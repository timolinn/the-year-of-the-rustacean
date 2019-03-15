#![allow(unused)]
#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(x: u8, y: u8) -> u8 {
    x + y
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)] // (configuration) tells the rust compiler not to compile code during build or run except during test.
mod tests {
    use crate::*;
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn rectangle_can_hold_another_rectangle() {
        let rect = Rectangle { width: 30, height: 10 };
        assert!(rect.can_hold(&Rectangle { width: 20, height:2 }));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[ignore]
    fn it_should_add_both_values() {
        assert_eq!(15, add(7, 8));
        assert_ne!(14, add(7, 8));
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100, got 300.")]
    fn greater_than_100() {
        Guess::new(300);
    }
}
