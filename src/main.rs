use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Score", 100);

    // if let Some(score) = scores.get_mut("Score") {
    //     *score += 50;
    //     println!("Score: {}", score);
    // } else {
    //     println!("Score not found");
    // }

    scores.entry("Score").and_modify(|e| *e += 50);
    println!("Score: {:?}", scores);
}
