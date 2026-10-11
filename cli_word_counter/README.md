# 🦀 Rust CLI Word Counter — A Beginner-Friendly Rust Learning Project

A simple CLI mini project for learning Rust by building a word counter. It reads a
text file (or text piped into it), counts lines, words, and characters, and shows
the most frequent words.

---

## Features

| Feature | Description |
|---|---|
| **Text statistics** | Counts lines, words, characters, and unique words |
| **Most frequent words** | Shows the top N words (default 5, change it with `--top`) |
| **File or stdin** | Reads a file, or standard input when no file is given or the file is `-` |
| **Case-insensitive** | `Rust`, `rust`, and `RUST,` count as the same word |
| **Unicode support** | Works with accents and non-Latin text (`café`, `日本語`) |
| **Clear errors** | Missing files, bad options, and invalid UTF-8 give a message instead of a crash |
| **Unit tests** | 21 built-in tests using Rust's testing framework |

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
cd cli_word_counter
cargo run -- sample.txt
```

Everything after `--` is passed to the program instead of to Cargo.

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

```
cli_word_counter [OPTIONS] [FILE]
```

| Option | Description |
|---|---|
| `-n`, `--top <N>` | Number of most frequent words to show (default: 5) |
| `-h`, `--help` | Show the help message |

**Count a file** (a `sample.txt` is included):

```bash
cargo run -- sample.txt
```

```
Source:       sample.txt
Lines:        6
Words:        67
Characters:   391
Unique words: 52

Top 5 words:
   1. a    5
   2. and  3
   3. is   3
   4. the  3
   5. to   3
```

**Show more or fewer words:**

```bash
cargo run -- sample.txt --top 10
```

**Read from standard input:**

```bash
echo "to be or not to be" | cargo run -- - --top 2
```

```
Source:       <stdin>
Lines:        1
Words:        6
Characters:   19
Unique words: 4

Top 2 words:
   1. be  2
   2. to  2
```

**How things are counted:**

- A **word** is a piece of text between spaces or line breaks, with punctuation removed from both ends. Punctuation inside a word stays (`don't`), and pieces with no letters or digits (such as `-` or `...`) are not words.
- **Characters** are Unicode characters, including spaces and line breaks.
- Words with the same count are listed alphabetically, so the output is the same on every run.

**Exit codes:** `0` on success, `1` if the input cannot be read, `2` if the command line is wrong.

---

## Project Structure

```
cli_word_counter/
├── Cargo.toml       # Project configuration and dependencies
├── README.md        # This documentation
├── sample.txt       # Example text to try the program with
└── src/
    └── main.rs      # The whole program
```

There are no external dependencies — this project only uses the Rust standard library.

---

## Rust Concepts Covered

### 1. Command-Line Arguments (`std::env`)
```rust
env::args_os().skip(1)
```
`env::args_os()` gives the arguments the program was started with. The first one is
the program name, so we `skip(1)`. We use `args_os()` instead of `args()` because
`args()` panics on text that is not valid Unicode, and we want a normal error message.

---

### 2. `struct` — Grouping Related Data
```rust
struct Config {
    path: Option<String>,
    top: usize,
    show_help: bool,
}
```
`Config` holds what the user asked for and `Stats` holds the numbers we calculate.
Small structs keep function signatures short and easy to read.

---

### 3. `Option<T>` — A Value That May Not Exist
```rust
path: Option<String>   // Some("notes.txt") or None (read from stdin)
```
`None` means "no file given". The compiler forces us to handle both cases.
`config.path.as_deref()` turns `Option<String>` into `Option<&str>` so we can
borrow the text without cloning it.

---

### 4. `Result` and the `?` Operator
```rust
let text = fs::read_to_string(file)
    .map_err(|error| format!("cannot read '{file}': {error}"))?;
```
`read_to_string` returns a `Result`. `map_err` rewrites the error into a friendly
message, and `?` returns early from the function if there was an error. Only
`main()` decides how errors are shown and which exit code to use.

---

### 5. Reading Files (`std::fs`)
```rust
let text = fs::read_to_string("sample.txt")?;
```
One call reads a whole file into a `String`. It returns an error for a missing file,
a directory, or text that is not valid UTF-8 — all handled without crashing.

---

### 6. `HashMap` — Counting Things
```rust
let mut frequencies: HashMap<String, usize> = HashMap::new();
*frequencies.entry(word).or_insert(0) += 1;
```
A `HashMap` stores `word -> count` pairs. `entry(...).or_insert(0)` returns the
current count, starting at 0 for a new word, and `*` lets us add 1 to it.

---

### 7. Iterators and Closures
```rust
for word in text.split_whitespace().filter_map(normalize) {
    // ...
}
```
`split_whitespace()` yields each piece of text, and `filter_map` keeps only the
pieces that `normalize` turns into a word (`Some`) and drops the rest (`None`).
Closures like `|a, b| b.1.cmp(&a.1)` are small inline functions used for sorting.

---

### 8. Sorting with a Custom Order
```rust
ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
ranked.truncate(top);
```
A `HashMap` has no order, so we move the pairs into a `Vec` and sort: highest count
first, and alphabetical order when counts are equal. Then `truncate` keeps the top N.

---

### 9. Exit Codes (`std::process`)
```rust
process::exit(2); // wrong usage
```
Programs tell the shell whether they succeeded through an exit code. `0` means
success; anything else means a problem. Scripts can use this to react to errors.

---

### 10. Unit Testing Built In
```rust
#[test]
fn test_analyze_is_case_insensitive() {
    let stats = analyze("Rust rust RUST,", 5);
    assert_eq!(stats.unique_words, 1);
}
```
Because `main()` only wires functions together, every piece of logic can be tested
without touching the keyboard, the screen, or the real command line. Run the tests
with `cargo test`.

---

## Known Limitations

- The whole input is read into memory, so extremely large files need a lot of RAM.
- Columns are aligned by character count, so wide characters such as `日本語` can look slightly misaligned in a terminal.
- Words are split on whitespace, so languages written without spaces (such as Japanese) are counted as one word per space-separated piece.

---

## Next Steps

After understanding this project, you can try:

1. **Several files** — accept more than one file and print a total, like the `wc` command
2. **Ignore common words** — add a `--ignore` option or a stop-word list ("the", "a", "and")
3. **Streaming** — read line by line with `BufRead` so large files do not need to fit in memory
4. **Sort options** — add `--sort alpha` to sort by word instead of by count
5. **Better argument parsing** — learn the `clap` crate and compare it with the hand-written `parse_args`
6. **Aligned output** — use the `unicode-width` crate to align wide characters
