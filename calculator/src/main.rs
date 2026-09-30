use std::io;

fn read_input() -> f64 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().parse().expect("Please enter a number")
}

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    loop {
        println!("--- Calculator ---");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("-----------------");
        println!("Choose an option (1-4):");

        let choice = read_input();

        println!("Enter first number:");
        let first = read_input();

        println!("Enter second number:");
        let second = read_input();

        match choice as i32 {
            1 => println!("Result: {}", add(first, second)),

            2 => println!("Result: {}", subtract(first, second)),

            3 => println!("Result: {}", multiply(first, second)),

            4 => match divide(first, second) {
                Some(result) => println!("Result: {}", result),
                None => println!("Division by zero is not allowed."),
            },

            _ => println!("Invalid option."),
        }

        println!();
    }
}
