pub fn mean(list: &Vec<i32>) -> i32 {
    let sum = list.iter().map(|&x: &i32| x).sum();
    sum
}

// pub fn median(list: &Vec<i32>) -> i32 {
//     let sorted = list.sort();
//     if ((list.len() % 2) == 0)
//     let medianIndex = /2;
// }

// pub fn mode(list: &Vec<i32>) -> i32 {
// }