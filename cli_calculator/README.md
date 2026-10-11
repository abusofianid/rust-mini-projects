# 🦀 Rust CLI Calculator — A Beginner-Friendly Rust Learning Project

A simple CLI mini project for learning the fundamentals of the Rust programming
language by building a calculator that runs in the terminal.

---

## Features

| Feature | Description |
|---|---|
| **Basic operations** | Addition (`+`), subtraction (`-`), multiplication (`*`), and division (`/`) |
| **Decimal numbers** | All numbers are processed as `f64`, so `3.5 * 2` works too |
| **Input validation** | Invalid numbers or operators are asked for again, the program does not crash |
| **Division by zero** | Prints a warning and returns `inf` |
| **Repeat calculations** | After each calculation, answer `y` to keep going |

---

## How to Run

### Prerequisites

Make sure Rust is installed (this project uses Rust edition 2024, so you need Rust 1.85 or newer):

```bash
cargo --version
```

If it is not installed, get it from [https://rustup.rs](https://rustup.rs).

### Run the App

```bash
cd cli_calculator
cargo run
```

### Format the Code

```bash
cargo fmt
```

### Check for Errors Without a Full Build

```bash
cargo check
```

---

## How to Use

After `cargo run`, the program asks for two numbers and one operator:

```
=== Simple CLI Calculator ===
=== Simple CLI Calculator ===
Enter the first number: 10
Enter an operator (+, -, *, /): *
Enter the second number: 5
Result: 10 * 5 = 50
Do you want to perform another calculation? (y/n): n
Thank you for using the calculator. Goodbye!
```

**Example flow:**

1. Enter the first number, then press Enter
2. Enter an operator (`+`, `-`, `*`, or `/`), then press Enter
3. Enter the second number, then press Enter
4. The result is shown right away
5. Type `y` to calculate again, or any other key to exit

**Example of invalid input:**

```
Enter the first number: abc
Invalid number. Please try again.
Enter an operator (+, -, *, /): x
Invalid operator. Please enter one of +, -, *, /
```

---

## Project Structure

```
cli_calculator/
├── Cargo.toml       # Project configuration and dependencies
├── README.md        # This documentation
└── src/
    └── main.rs      # The whole program
```

There are no external dependencies — this project only uses the Rust standard library.

---

## Rust Concepts Covered

### 1. Functions with Parameter and Return Types
```rust
fn calculate(a: f64, b: f64, operator: char) -> f64 { ... }
```
Rust requires parameter types and return types to be written explicitly. `f64` is a
64-bit floating-point number, while `char` is a single character.

---

### 2. `loop`, `break`, and `return`
```rust
loop {
    // ... ask for input ...
    if continue_input.trim().to_lowercase() != "y" {
        break;
    }
}
```
`loop` runs forever until a `break` (exit the loop) or a `return` (exit the function).
This project uses `loop` twice: to repeat the calculator, and to keep asking for
input until it is valid.

---

### 3. Reading Terminal Input (`stdin`)
```rust
let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
```
`read_line` writes the input into a `String` that we lend out mutably (`&mut input`).
The result always includes the trailing newline, so we call `.trim()` before
processing it.

---

### 4. Showing a Prompt with `flush`
```rust
print!("{}", prompt);
let _ = io::Write::flush(&mut io::stdout());
```
`print!` (without `ln`) does not print a newline, and its output may stay in a
buffer. `flush` forces the prompt to appear before the program waits for input.

---

### 5. `String` vs `&str`
```rust
fn read_number(prompt: &str) -> f64 { ... }
```
`String` is owned, modifiable text. `&str` is a borrowed reference to text that
already exists. For function parameters that only read text, use `&str`.

---

### 6. `Result` and Error Handling
```rust
match input.trim().parse::<f64>() {
    Ok(v)  => return v,
    Err(_) => println!("Invalid number. Please try again."),
}
```
`parse()` returns a `Result`. With `match`, we handle success (`Ok`) and failure
(`Err`) explicitly — there are no hidden exceptions.

---

### 7. `match` — Pattern Matching
```rust
match operator {
    '+' => a + b,
    '-' => a - b,
    '*' => a * b,
    '/' => { /* ... */ }
    _   => 0.00,
}
```
`match` picks a branch based on the value. `_` is the wildcard for any other value,
and the compiler makes sure every possibility is handled.

---

### 8. `if` as an Expression and `f64::INFINITY`
```rust
if b == 0.0 {
    println!("Warning: Division by zero detected. Returning infinity.");
    f64::INFINITY
} else {
    a / b
}
```
In Rust, `if/else` produces a value, so it can be the final result of a `match`
branch. Floating-point numbers (`f64`) have a special `INFINITY` value, so dividing
by zero does not crash the program.

---

## Next Steps

After understanding this project, you can try:

1. **Unit tests** — add `#[cfg(test)]` to test the `calculate` function
2. **Better error handling** — replace `unwrap()` with `?` or `match`, and make `calculate` return `Result<f64, String>` for the division-by-zero case
3. **New operators** — add modulo (`%`) and power
4. **Calculation history** — store previous results in a `Vec<String>` and show them at the end
5. **Expression parser** — allow input like `2 + 3 * 4` on a single line
