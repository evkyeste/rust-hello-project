use std::collections::HashMap;

fn main() {
    let mut fruits = HashMap::new();
    fruits.insert("Apple", 5);
    fruits.insert("Banana", 3);

    if let Some(&count) = fruits.get("Apple") {
        println!("Apple count: {}", count); // Output: Apple count: 5
    }
}
