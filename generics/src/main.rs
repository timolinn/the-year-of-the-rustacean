#![allow(unused)]
enum Option<T> {
    Some(T),
    None
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list.iter() {
        if item.clone() > largest {
            largest = item.clone();
        }
    }

    largest
}

fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }

    &largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest::<i32>(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest::<char>(&char_list);
    println!("The largest char is {}", result);


    let naija: Point<i32, f32> = Point { x: 4, y: 14.0 };
}
