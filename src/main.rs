use std::io;

fn main() {
    println!("🦀 Rust CLI Calculator");

    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut num1).expect("Failed to read");

    println!("Enter operator (+ - * / %):");
    io::stdin().read_line(&mut operator).expect("Failed to read");

    println!("Enter second number:");
    io::stdin().read_line(&mut num2).expect("Failed to read");

    let num1: f64 = num1.trim().parse().expect("Invalid number");
    let num2: f64 = num2.trim().parse().expect("Invalid number");
    let operator = operator.trim();

    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        "%" => num1 % num2,
        _ => {
            println!("Invalid operator!");
            return;
        }
    };

    println!("Result: {}", result);
}