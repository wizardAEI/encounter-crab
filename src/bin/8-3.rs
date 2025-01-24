// Storing Keys with Associated Values in Hash Maps
use std::collections::HashMap;
fn main() {

    let mut scores = HashMap::new();

    scores.insert("Blue", 10);
    let yellow_score = scores.entry("Yellow").and_modify(|e| *e += 1).or_insert(50);
    *yellow_score += 1;

    let team_name = "Yellow";

    let score = scores.get(team_name);

    if let Some(v) = score {
        println!("score: {}",v);
    }    

    scores.entry("Yellow").and_modify(|e| *e += 1);
    scores.insert("Blue", 50);

    println!("scores {:#?}", scores)
}