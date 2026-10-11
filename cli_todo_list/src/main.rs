// ============================================================
// rust-todo: A CLI Todo List app for learning Rust
// ============================================================
//
// Rust concepts covered in this file:
//   - struct      : define a custom data type
//   - enum        : a type with several possible values
//   - Vec<T>      : dynamic array (growable list)
//   - match       : pattern matching
//   - impl        : add methods to a struct / enum
//   - ownership   : String vs &str
//   - Result / unwrap_or : lightweight error handling
//   - format!     : format text like printf
//   - fn          : define functions
// ============================================================

use std::io::{self, Write};

// ----------------------------------------------------------
// ENUM: The status of a task.
//
// An `enum` in Rust defines a type that can only be one of
// its listed variants.
//
// `#[derive(Debug)]` lets Rust automatically print the value
// of this enum for debugging purposes.
// ----------------------------------------------------------
#[derive(Debug)]
enum TaskStatus {
    Pending,
    Completed,
}

// Add a method to TaskStatus so it can be shown as text.
impl TaskStatus {
    /// Returns an easy-to-read text label.
    fn label(&self) -> &str {
        // `match` makes sure every variant is handled — the compiler
        // gives an error if a variant is missed.
        match self {
            TaskStatus::Pending => "[ ]",
            TaskStatus::Completed => "[x]",
        }
    }
}

// ----------------------------------------------------------
// STRUCT: The representation of a task.
//
// A `struct` groups several related fields into a single
// new data type — like a class without inheritance.
// ----------------------------------------------------------
struct Task {
    id: u32,       // Unique ID, of type u32 (unsigned 32-bit integer)
    title: String, // Task title — String is owned text
    status: TaskStatus,
}

// Add methods to Task so it can print a summary line.
impl Task {
    /// Creates a new Task with the Pending status by default.
    fn new(id: u32, title: String) -> Task {
        Task {
            id,
            title,
            status: TaskStatus::Pending,
        }
    }

    /// Prints a one-line summary of the task.
    fn display(&self) {
        // `&self` means the method borrows the Task without taking ownership of it.
        println!(
            "  {}  [ID: {:>3}]  {}",
            self.status.label(),
            self.id,
            self.title
        );
    }
}

// ----------------------------------------------------------
// MAIN FUNCTION: main() — the entry point of a Rust program
// ----------------------------------------------------------
fn main() {
    // `Vec<Task>` is a list of tasks that can grow dynamically.
    let mut tasks: Vec<Task> = Vec::new();

    // Counter for the next task ID.
    let mut next_id: u32 = 1;

    println!("==============================");
    println!("   Rust Todo List  🦀");
    println!("==============================");

    // Main program loop — keeps running until the user chooses Exit.
    loop {
        print_menu();

        let choice = read_input("Choose a menu option (1-5): ");

        // `match` here handles each menu choice.
        // `trim()` removes spaces/newlines at the start & end of the input.
        match choice.trim() {
            "1" => add_task(&mut tasks, &mut next_id),
            "2" => list_tasks(&tasks),
            "3" => complete_task(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                println!("\nGoodbye! 👋");
                break; // Exit the loop
            }
            // `_` is a wildcard — it catches every other value.
            _ => println!("\n⚠️  Invalid choice. Enter a number from 1 to 5.\n"),
        }
    }
}

// ----------------------------------------------------------
// Prints the menu options to the screen.
// ----------------------------------------------------------
fn print_menu() {
    println!("\n------------------------------");
    println!(" MENU");
    println!("------------------------------");
    println!(" 1. Add Task");
    println!(" 2. List Tasks");
    println!(" 3. Complete Task");
    println!(" 4. Delete Task");
    println!(" 5. Exit");
    println!("------------------------------");
}

// ----------------------------------------------------------
// Reads one line of input from the user.
//
// The parameter `prompt: &str` uses a reference (&) to a string —
// it neither copies nor owns that string.
// The function returns a String (owned).
// ----------------------------------------------------------
fn read_input(prompt: &str) -> String {
    // Print the prompt without a newline so the cursor stays on the same line.
    print!("{}", prompt);

    // `flush()` makes sure the prompt text is actually shown before reading input.
    io::stdout().flush().unwrap_or(());

    let mut input = String::new();

    // `io::stdin().read_line()` reads a line from the keyboard.
    // If it fails (e.g. a closed pipe), return an empty string.
    io::stdin().read_line(&mut input).unwrap_or(0);

    input
}

// ----------------------------------------------------------
// Adds a new task to the list.
//
// `tasks: &mut Vec<Task>` — borrows the Vec mutably
//   so we can add elements.
// `next_id: &mut u32` — borrows the counter so it can be changed.
// ----------------------------------------------------------
fn add_task(tasks: &mut Vec<Task>, next_id: &mut u32) {
    let input = read_input("\nNew task title: ");
    let title = input.trim();

    // Validation: the title must not be empty.
    if title.is_empty() {
        println!("⚠️  The task title cannot be empty.");
        return; // Exit the function early
    }

    // `to_string()` turns a &str into a String (owned copy).
    let task = Task::new(*next_id, title.to_string());

    println!("✅  Task '{}' added (ID: {}).", task.title, task.id);

    tasks.push(task); // Add it to the Vec
    *next_id += 1; // `*` writes to the value behind the reference (dereference)
}

// ----------------------------------------------------------
// Shows all existing tasks.
//
// `tasks: &Vec<Task>` — borrows the Vec read-only (immutable).
// ----------------------------------------------------------
fn list_tasks(tasks: &Vec<Task>) {
    println!();

    if tasks.is_empty() {
        println!("📋  No tasks yet. Add one by choosing option 1.");
        return;
    }

    let total = tasks.len();
    let done = tasks
        .iter()
        .filter(|t| matches!(t.status, TaskStatus::Completed))
        .count();

    println!("📋  Task List ({}/{} done):", done, total);
    println!("  --------------------------------");

    // The `for` loop iterates over each &Task in the Vec (a reference, not a copy).
    for task in tasks {
        task.display();
    }

    println!("  --------------------------------");
}

// ----------------------------------------------------------
// Marks a task as Completed.
// ----------------------------------------------------------
fn complete_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("\n⚠️  No tasks yet.");
        return;
    }

    list_tasks(tasks);

    let input = read_input("Enter the ID of the task to complete: ");

    // `parse::<u32>()` turns a String into a u32 number.
    // The result is a Result<u32, _>; we handle the error with a clear message.
    let id: u32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("⚠️  Invalid ID. Enter a number.");
            return;
        }
    };

    // `iter_mut()` gives a mutable reference to each element.
    for task in tasks.iter_mut() {
        if task.id == id {
            match task.status {
                TaskStatus::Completed => {
                    println!("ℹ️  Task '{}' is already completed.", task.title);
                }
                TaskStatus::Pending => {
                    task.status = TaskStatus::Completed;
                    println!("✅  Task '{}' marked as completed!", task.title);
                }
            }
            return;
        }
    }

    println!("⚠️  Task with ID {} not found.", id);
}

// ----------------------------------------------------------
// Removes a task from the list.
// ----------------------------------------------------------
fn delete_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("\n⚠️  No tasks to delete.");
        return;
    }

    list_tasks(tasks);

    let input = read_input("Enter the ID of the task to delete: ");

    let id: u32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("⚠️  Invalid ID. Enter a number.");
            return;
        }
    };

    // `position()` finds the index of the first element matching the condition.
    // The result is an Option<usize>: Some(index) if found, None if not.
    let pos = tasks.iter().position(|t| t.id == id);

    match pos {
        Some(index) => {
            // `remove(index)` removes the element at that index
            // and returns the removed value (owned).
            let removed = tasks.remove(index);
            println!("🗑️  Task '{}' deleted successfully.", removed.title);
        }
        None => {
            println!("⚠️  Task with ID {} not found.", id);
        }
    }
}

// ----------------------------------------------------------
// AUTOMATED TESTS
//
// Rust has a built-in testing system.
// Run it with: cargo test
//
// The `#[cfg(test)]` block is only compiled during `cargo test`.
// ----------------------------------------------------------
#[cfg(test)]
mod tests {
    // `super::*` imports every item from the parent module (this file).
    use super::*;

    #[test]
    fn test_new_task_is_pending() {
        let task = Task::new(1, "Learn Rust".to_string());

        // Make sure the initial status is Pending.
        assert!(matches!(task.status, TaskStatus::Pending));
        assert_eq!(task.id, 1);
        assert_eq!(task.title, "Learn Rust");
    }

    #[test]
    fn test_status_label_pending() {
        let status = TaskStatus::Pending;
        assert_eq!(status.label(), "[ ]");
    }

    #[test]
    fn test_status_label_completed() {
        let status = TaskStatus::Completed;
        assert_eq!(status.label(), "[x]");
    }

    #[test]
    fn test_add_task_to_vec() {
        let mut tasks: Vec<Task> = Vec::new();
        let mut next_id: u32 = 1;

        // Simulate add_task directly (without keyboard input).
        tasks.push(Task::new(next_id, "Task A".to_string()));
        next_id += 1;
        tasks.push(Task::new(next_id, "Task B".to_string()));

        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].id, 1);
        assert_eq!(tasks[1].id, 2);
    }

    #[test]
    fn test_complete_task_changes_status() {
        let mut tasks = vec![Task::new(1, "Read a book".to_string())];

        // Change the status of the first task to Completed.
        tasks[0].status = TaskStatus::Completed;

        assert!(matches!(tasks[0].status, TaskStatus::Completed));
    }

    #[test]
    fn test_delete_task_reduces_count() {
        let mut tasks = vec![
            Task::new(1, "Task one".to_string()),
            Task::new(2, "Task two".to_string()),
        ];

        // Delete the task with ID 1.
        let pos = tasks.iter().position(|t| t.id == 1);
        if let Some(index) = pos {
            tasks.remove(index);
        }

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, 2);
    }

    #[test]
    fn test_delete_nonexistent_id() {
        let tasks = vec![Task::new(1, "Task one".to_string())];

        let pos = tasks.iter().position(|t| t.id == 99); // ID does not exist
        assert!(pos.is_none()); // Not found = None
        assert_eq!(tasks.len(), 1); // The Vec is unchanged
    }
}
