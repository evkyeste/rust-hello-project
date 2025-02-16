use std::collections::BTreeMap;

fn main() {
    let mut numbers = BTreeMap::new();
    numbers.insert(1, "One");
    numbers.insert(2, "Two");
    numbers.insert(3, "Three");

    for (key, value) in &numbers {
        println!("{}: {}", key, value)
    }

    println!("{:?}", numbers)
}
