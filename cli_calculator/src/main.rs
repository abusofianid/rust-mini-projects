use core::f64;
use std::io;

// Definition of the main function, the entry point of the program
fn main() {
    println!("=== Simple CLI Calculator ===");
    loop {
        // Print the calculator header
        println!("=== Simple CLI Calculator ===");
        // Read the first number by calling the read_number function
        let number1 = read_number("Enter the first number: ");
        // Read the operator by calling the read_operator function
        let operator = read_operator("Enter an operator (+, -, *, /): ");
        // Read the second number by calling the read_number function
        let number2 = read_number("Enter the second number: ");
        // Compute the result by calling the calculate function
        let result = calculate(number1, number2, operator);
        // Show the calculation result
        println!("Result: {} {} {} = {}", number1, operator, number2, result);

        // Initialize an empty string to store the user's answer about continuing
        let mut continue_input = String::new();
        print!("Do you want to perform another calculation? (y/n): ");
        // Flush stdout so the prompt is shown right away before the program waits for input
        let _ = io::Write::flush(&mut io::stdout());
        // Read a line from stdin into continue_input, using unwrap to handle errors
        io::stdin().read_line(&mut continue_input).unwrap();
        // If the trimmed, lowercased input is not "y", exit the loop
        if continue_input.trim().to_lowercase() != "y" {
            break;
        }
    }
    println!("Thank you for using the calculator. Goodbye!");
}

// Definition of read_number, which takes a prompt as a string slice and returns an f64
fn read_number(prompt: &str) -> f64 {
    // Start an infinite loop to keep asking until the input is valid
    loop {
        // Initialize an empty string to store the user's input
        let mut input = String::new();
        // Print the prompt without a newline so the input follows it directly
        print!("{}", prompt);
        // Flush stdout so the prompt is shown right away before reading input
        let _ = io::Write::flush(&mut io::stdout());
        // Read a line from stdin into input, using unwrap to handle errors
        io::stdin().read_line(&mut input).unwrap();
        // Match on the result of parsing the trimmed input as an f64
        match input.trim().parse::<f64>() {
            // If parsing succeeds, return the f64 value
            Ok(v) => return v,
            // If parsing fails, print an error message and continue the loop
            Err(_) => {
                println!("Invalid number. Please try again.");
            }
        }
    }
}

// Definition of read_operator, which takes a prompt as a string slice and returns a char
fn read_operator(prompt: &str) -> char {
    // Start an infinite loop to keep asking until the operator is valid
    loop {
        // Initialize an empty string to store the user's input
        let mut input = String::new();
        // Print the prompt without a newline so the input follows it directly
        print!("{}", prompt);
        // Flush stdout so the prompt is shown right away before reading input
        let _ = io::Write::flush(&mut io::stdout());
        // Read a line from stdin into input, using unwrap to handle errors
        io::stdin().read_line(&mut input).unwrap();
        // Trim the input and store it as the operator
        let operator = input.trim();
        // Check that the operator is exactly 1 byte long
        if operator.len() == 1 {
            // Take the first character of the operator
            let c = operator.chars().next().unwrap();
            // Check that the character is one of +, -, *, /
            if "+-*/".contains(c) {
                // If so, return that character
                return c;
            }
        }
        println!("Invalid operator. Please enter one of +, -, *, /");
    }
}

// Definition of calculate, which takes two f64 values and an operator char, and returns the f64 result
fn calculate(a: f64, b: f64, operator: char) -> f64 {
    // Use match to choose the operation based on the operator
    match operator {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => {
            if b == 0.0 {
                println!("Warning: Division by zero detected. Returning infinity.");
                f64::INFINITY
            } else {
                a / b
            }
        }
        // For any other unknown operator, return 0.0 as a default
        _ => 0.00,
    }
}
