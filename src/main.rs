use std::io;

fn square(n: i64) -> Result<i64, String> {
    if n < 0 {
        Err("Input must be non-negative".to_string())
    } else {
        Ok(n * n)
    }
}

fn main() {
    loop {
        println!("Enter a number:");

        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            println!("Failed to read input: {}", e);
            continue;
        }

        let number: Result<i64, _> = input.trim().parse();

        match number {
            Ok(num) => match square(num) {
                Ok(result) => {
                    println!("The square of {} is {}", num, result);
                    break;
                }
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            },
            Err(_) => {
                println!("Invalid input! Please enter a valid number.");
                continue;
            }
        }
    }
}
