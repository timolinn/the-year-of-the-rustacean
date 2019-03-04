fn main() {
    hash_map();
    update_kv();
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

fn update_kv() -> () {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
