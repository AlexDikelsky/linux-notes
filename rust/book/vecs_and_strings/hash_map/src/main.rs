use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }



    let name = String::from("Fav Color");
    let value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(name, value);

    //println!("{}", name);
    //Causes an error because name is now owned by the HashMap
}
