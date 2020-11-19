use std::collections::HashMap;
fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{}", field_name);
    // println!("{}", field_value);

    let field_name = String::from("Favorite color");
    let field_value = "Blue";

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{}", field_value);

    let mut scores = HashMap::new();

    let blue_score = 10;
    let yellow_score = 50;
    scores.insert(String::from("Blue"), blue_score);
    scores.insert(String::from("Yellow"), yellow_score);
    println!("{}", blue_score);
    println!("{}", yellow_score);

    println!("{:?}", scores);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
    let return_blue = scores.entry(String::from("Blue")).or_insert(50);
    println!("{}", return_blue);
    let return_red = scores.entry(String::from("Red")).or_insert(100);
    println!("{}", return_red);
    println!("{:?}", scores);
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
