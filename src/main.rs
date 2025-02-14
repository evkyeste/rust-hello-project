fn main() {
    let person: (&str, i32) = ("Alice", 25);
    let (name, age) = person;
    println!("Name = {}, Age = {}", name, age);
}
