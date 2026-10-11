# 🦀 Rust Todo List — A Beginner-Friendly Rust Learning Project

A simple CLI mini project for learning the fundamentals of the Rust programming
language by building a Todo List app that runs in the terminal.

---

## Features

| Feature | Description |
|---|---|
| **Add Task** | Add a new task with a title |
| **List Tasks** | Show all tasks along with their status (done / not done) |
| **Complete Task** | Mark a task as done by its ID |
| **Delete Task** | Remove a task from the list by its ID |
| **Input validation** | The program does not crash on invalid input |
| **Unit tests** | 7 built-in tests using Rust's testing framework |

---

## How to Run

### Prerequisites

Make sure Rust is installed:

```bash
cargo --version
```

If it is not installed, get it from [https://rustup.rs](https://rustup.rs).

### Run the App

```bash
cd cli_todo_list
cargo run
```

### Run the Tests

```bash
cargo test
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

After `cargo run`, you will see a menu like this:

```
==============================
   Rust Todo List  🦀
==============================

------------------------------
 MENU
------------------------------
 1. Add Task
 2. List Tasks
 3. Complete Task
 4. Delete Task
 5. Exit
------------------------------
Choose a menu option (1-5):
```

**Example flow:**

1. Type `1` → press Enter → type a task title → press Enter
2. Type `2` → see all tasks with their ID and status
3. Type `3` → enter the ID of the task you want to complete
4. Type `4` → enter the ID of the task you want to delete
5. Type `5` → exit the program

---

## Project Structure

```
cli_todo_list/
├── Cargo.toml       # Project configuration and dependencies
├── README.md        # This documentation
└── src/
    └── main.rs      # The whole program
```

---

## Rust Concepts Covered

### 1. `struct` — Custom Data Types
```rust
struct Task {
    id: u32,
    title: String,
    status: TaskStatus,
}
```
A `struct` groups several fields into a single type. It is similar to a class in
other languages, but without inheritance.

---

### 2. `enum` — A Type with Several Possible Values
```rust
enum TaskStatus {
    Pending,
    Completed,
}
```
An `enum` guarantees that a task's status can only be `Pending` or `Completed`,
never an arbitrary string. The compiler warns you if a variant is left unhandled.

---

### 3. `Vec<T>` — Dynamic Array
```rust
let mut tasks: Vec<Task> = Vec::new();
tasks.push(task);      // add an element
tasks.remove(index);   // remove by index
tasks.len();           // number of elements
```
`Vec` is like `ArrayList` in Java or `list` in Python — its size can grow at
runtime.

---

### 4. `match` — Pattern Matching
```rust
match choice.trim() {
    "1" => add_task(...),
    "2" => list_tasks(...),
    _ => println!("Invalid"),
}
```
`match` is safer than a long `if/else if` chain because the compiler makes sure
every possibility is handled. `_` is a wildcard that catches any other value.

---

### 5. `impl` — Methods for a Struct/Enum
```rust
impl Task {
    fn new(id: u32, title: String) -> Task { ... }
    fn display(&self) { ... }
}
```
`impl` adds functions to our own types. `&self` means the method borrows the struct
without taking ownership of it.

---

### 6. Ownership & References (`&` and `&mut`)
```rust
fn list_tasks(tasks: &Vec<Task>)          // borrow read-only
fn add_task(tasks: &mut Vec<Task>, ...)   // borrow to modify
```
This is a concept unique to Rust. Instead of copying data or using a garbage
collector, Rust borrows data under strict rules:
- `&T` — can be read, cannot be modified
- `&mut T` — can be read and modified, but only one mutable borrower is allowed at a time

---

### 7. `String` vs `&str`
```rust
let owned: String = "text".to_string();   // String — owned, lives on the heap
fn prompt(s: &str) { ... }                // &str  — a reference to text, not owned
```
`String` is owned text that can be modified. `&str` is a borrowed reference to text
that already exists. Use `&str` for function parameters that only read text.

---

### 8. `Result` and Error Handling
```rust
let id: u32 = match input.trim().parse() {
    Ok(n)  => n,
    Err(_) => { println!("Not a number!"); return; }
};
```
`parse()` returns a `Result<u32, _>`. With `match`, we handle success (`Ok`) and
failure (`Err`) explicitly — there are no hidden exceptions like in Python/Java.

---

### 9. `Option` — A Value That May Not Exist
```rust
let pos: Option<usize> = tasks.iter().position(|t| t.id == id);
match pos {
    Some(index) => tasks.remove(index),
    None        => println!("Not found"),
}
```
`Option` replaces `null`. The compiler forces us to handle the "no value" case, so
there are no NullPointerExceptions.

---

### 10. Built-in Unit Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_task_is_pending() {
        let task = Task::new(1, "Learn Rust".to_string());
        assert!(matches!(task.status, TaskStatus::Pending));
    }
}
```
Rust has a built-in testing framework. Just write a function marked with `#[test]`
and run `cargo test`. The `#[cfg(test)]` block makes sure test code is not
compiled into production builds.

---

## Next Steps

After understanding this project, you can try:

1. **Save to a file** — use `std::fs::write` and `std::fs::read_to_string`
2. **Add a due date** — add a `due_date: Option<String>` field to `Task`
3. **Filter tasks** — show only Pending tasks or only Completed tasks
4. **Priorities** — add an enum `Priority { Low, Medium, High }`
5. **Serialization** — learn the `serde` + `serde_json` crates to save to JSON

---

## License

MIT — free to use for learning.
