#![allow(unused)]
fn main() {
    hash_map();
    update_key_value();
    get_highest_value();
}


fn hash_map() -> () {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 30);
    scores.insert(String::from("Red"), 20);

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let team_blue_score = scores.get(&String::from("Blue")); // will retunr `Option<&V>`
    println!("The blue team is scoring {} points", team_blue_score.unwrap());

    // You can overwrite a value in a hashmap by simply
    // inserting another value with an existing key - value
    scores.insert(String::from("Blue"), 100);
    println!("{:?}", scores);

    // Or ignore if key already has a value
    scores.entry(String::from("Blue")).or_insert(300);
    println!("{:?}", scores);

}

fn update_key_value() -> () {
    use std::collections::HashMap;

    let text = vec![1, 2, 3, 4, 1,3, 4, 1];

    let mut map = HashMap::new();

    for word in text {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn get_highest_value() -> () {
    use std::collections::HashMap;

    let numbers = vec![1, 2, 3, 4, 1,3, 3, 3, 4, 1];
    let mut map = HashMap::new();
    for number in numbers {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }
    let mut max_value: Option<(&u32, &u32)> = None;
    for (key, value) in map.iter() {
        max_value = match max_value {
            None => Some((key, value)),
            Some(max_value) => if value > &max_value.1 {
                Some((key, value))
            } else {
                Some(max_value)
            },
        };
    }
    println!("{}", max_value.unwrap().0);
}
