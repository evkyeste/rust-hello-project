use std::collections::HashMap;

fn main() {
    let mut colors = HashMap::new();
    colors.insert("Red", 10);
    colors.insert("Blue", 20);

    for (color, value) in colors {
        println!("Color: {} - {}", color, value);
    }
}
