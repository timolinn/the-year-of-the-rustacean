pub fn get_highest_value_folding() -> () {
    use std::collections::HashMap;

    let numbers = vec![1, 2, 3, 4, 1,3, 3, 3, 4, 1];
    // let mut map = HashMap::new();
    // // for number in numbers {
    // //     let count = map.entry(number).or_insert(0);
    // //     *count += 1;
    // // }
    // convert into a vector of tuples
    let new_map = numbers.into_iter().fold(None, |max, item| match max {
        None => Some(item),
        Some(cur) => Some(if item > cur { item } else { cur }),
    });
    println!("{}", new_map.unwrap());
}

pub fn int_odd_number() -> () {
    use std::collections::HashMap;

    let numbers = vec![20,1,-1,2,-2,3,3,1,5,5, 2,4,20,4,-1,-2,5];
    let mut map = HashMap::new();
    for number in numbers {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }
    // println!("{:?}", map);
    map.retain(|&key, &mut val| val % 2 != 0);
    // println!("{:?}", map.into_iter().fold(1,|num, item| item.0));
    map.keys().for_each(|key| println!("The integer with odd count is = {:?}",key));
}