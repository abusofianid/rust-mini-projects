// ============================================================
// cli_word_counter: A CLI word counter for learning Rust
// ============================================================
//
// Rust concepts covered in this file:
//   - std::env     : read command-line arguments
//   - std::fs      : read a file from disk
//   - Result and ? : pass errors up instead of crashing
//   - HashMap      : count how often each word appears
//   - iterators    : split, filter_map, enumerate, max
//   - struct       : group related data (Config, Stats)
//   - unit tests   : #[cfg(test)] and #[test]
// ============================================================

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, IsTerminal, Read, Write};
use std::process;

// How many of the most frequent words are shown when --top is not given.
const DEFAULT_TOP: usize = 5;

// The help text shown for --help and after an argument error.
const USAGE: &str = "\
Usage: cli_word_counter [OPTIONS] [FILE]

Counts lines, words, and characters in a text file and shows the most
frequent words. Reads from standard input when FILE is omitted or is '-'.

Options:
  -n, --top <N>   Number of most frequent words to show (default: 5)
  -h, --help      Show this help message

Examples:
  cargo run -- sample.txt
  cargo run -- sample.txt --top 10
  cat sample.txt | cargo run -- -";

// ----------------------------------------------------------
// STRUCT: What the user asked for on the command line.
// ----------------------------------------------------------
#[derive(Debug, PartialEq)]
struct Config {
    path: Option<String>, // `None` means "read from standard input"
    top: usize,
    show_help: bool,
}

// ----------------------------------------------------------
// STRUCT: The numbers we calculate from the text.
// ----------------------------------------------------------
#[derive(Debug, PartialEq)]
struct Stats {
    lines: usize,
    words: usize,
    characters: usize,
    unique_words: usize,
    top_words: Vec<(String, usize)>, // (word, how many times it appears)
}

// ----------------------------------------------------------
// MAIN FUNCTION: main() — the entry point of a Rust program
//
// main() only wires things together. All the real work happens
// in small functions that are easy to test.
// ----------------------------------------------------------
fn main() {
    // `and_then` runs parse_args only if collect_args succeeded.
    let config = match collect_args().and_then(|args| parse_args(&args)) {
        Ok(config) => config,
        Err(message) => {
            eprintln!("Error: {message}\n\n{USAGE}");
            process::exit(2); // Exit code 2 = wrong usage
        }
    };

    if config.show_help {
        print_output(&format!("{USAGE}\n"));
        return;
    }

    match run(&config) {
        Ok(report) => print_output(&report),
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1); // Exit code 1 = something went wrong while running
        }
    }
}

// ----------------------------------------------------------
// Reads the command-line arguments (without the program name).
//
// `args_os()` is used instead of `args()` because `args()` would
// panic on text that is not valid Unicode. Here we turn that
// situation into a normal error message instead.
// ----------------------------------------------------------
fn collect_args() -> Result<Vec<String>, String> {
    env::args_os()
        .skip(1)
        .map(|arg| {
            arg.into_string()
                .map_err(|_| "arguments must be valid UTF-8".to_string())
        })
        // Collecting an iterator of Results gives one Result:
        // Ok(all values) or the first Err.
        .collect()
}

// ----------------------------------------------------------
// Turns a list of arguments into a Config.
//
// Takes `&[String]` (a borrowed slice) so tests can call it
// without touching the real command line.
// ----------------------------------------------------------
fn parse_args(args: &[String]) -> Result<Config, String> {
    let mut config = Config {
        path: None,
        top: DEFAULT_TOP,
        show_help: false,
    };

    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-h" | "--help" => config.show_help = true,
            "-n" | "--top" => {
                // The value is the next argument.
                // `ok_or_else` turns None into an Err; `?` returns early on Err.
                let value = iter
                    .next()
                    .ok_or_else(|| format!("missing value for {arg}"))?;
                config.top = value
                    .parse::<usize>()
                    .map_err(|_| format!("invalid number for {arg}: '{value}'"))?;
            }
            // Anything else that starts with '-' (except a lone '-') is an unknown option.
            _ if arg.starts_with('-') && arg.len() > 1 => {
                return Err(format!("unknown option: {arg}"));
            }
            // Everything else is the input file.
            _ => {
                if config.path.is_some() {
                    return Err("only one input file is supported".to_string());
                }
                config.path = Some(arg.clone());
            }
        }
    }

    Ok(config)
}

// ----------------------------------------------------------
// Runs the whole program and returns the report as text.
//
// Returning `Result<String, String>` means: Ok(the report) or
// Err(a message for the user). main() decides how to show them.
// ----------------------------------------------------------
fn run(config: &Config) -> Result<String, String> {
    let (source, text) = read_input(config.path.as_deref())?;
    let stats = analyze(&text, config.top);
    Ok(format_report(&source, &stats))
}

// ----------------------------------------------------------
// Reads the text to analyze and returns (source name, text).
//
// `path: Option<&str>` is either Some("file.txt") or None.
// A path of "-" also means "read standard input".
// ----------------------------------------------------------
fn read_input(path: Option<&str>) -> Result<(String, String), String> {
    match path {
        Some(file) if file != "-" => {
            let text = fs::read_to_string(file)
                .map_err(|error| format!("cannot read '{file}': {error}"))?;
            Ok((file.to_string(), text))
        }
        _ => {
            // With no FILE and a keyboard attached, waiting for input would
            // look like the program froze, so we explain what to do instead.
            if path.is_none() && io::stdin().is_terminal() {
                return Err(
                    "no input file given. Pass a FILE or pipe text into the program (see --help)"
                        .to_string(),
                );
            }

            let mut text = String::new();
            io::stdin()
                .read_to_string(&mut text)
                .map_err(|error| format!("cannot read standard input: {error}"))?;
            Ok(("<stdin>".to_string(), text))
        }
    }
}

// ----------------------------------------------------------
// Cleans one whitespace-separated piece of text into a word.
//
// - Removes punctuation at the start and end: "Hello," -> "hello"
// - Keeps punctuation inside a word: "don't" stays "don't"
// - Lowercases everything so "Rust" and "rust" count as the same word
// - Returns None when nothing is left (e.g. a lone "-" or "...")
// ----------------------------------------------------------
fn normalize(token: &str) -> Option<String> {
    let word = token.trim_matches(|c: char| !c.is_alphanumeric());
    if word.is_empty() {
        None
    } else {
        Some(word.to_lowercase())
    }
}

// ----------------------------------------------------------
// Counts lines, words, and characters, and finds the `top`
// most frequent words.
// ----------------------------------------------------------
fn analyze(text: &str, top: usize) -> Stats {
    // A HashMap stores pairs of (word -> count).
    let mut frequencies: HashMap<String, usize> = HashMap::new();
    let mut words = 0;

    // `filter_map` keeps only the tokens that `normalize` turns into words.
    for word in text.split_whitespace().filter_map(normalize) {
        words += 1;
        // `entry(...).or_insert(0)` gives the current count (starting at 0);
        // `*` lets us add 1 to the value behind the reference.
        *frequencies.entry(word).or_insert(0) += 1;
    }

    let unique_words = frequencies.len();

    // A HashMap has no order, so move the pairs into a Vec and sort it:
    // highest count first, and alphabetical order when counts are equal,
    // which keeps the output the same on every run.
    let mut ranked: Vec<(String, usize)> = frequencies.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    ranked.truncate(top);

    Stats {
        lines: text.lines().count(),
        words,
        characters: text.chars().count(),
        unique_words,
        top_words: ranked,
    }
}

// ----------------------------------------------------------
// Builds the text report that is printed to the screen.
// ----------------------------------------------------------
fn format_report(source: &str, stats: &Stats) -> String {
    let mut lines = vec![
        format!("Source:       {source}"),
        format!("Lines:        {}", stats.lines),
        format!("Words:        {}", stats.words),
        format!("Characters:   {}", stats.characters),
        format!("Unique words: {}", stats.unique_words),
    ];

    if stats.words == 0 {
        lines.push(String::new());
        lines.push("No words found.".to_string());
    } else if !stats.top_words.is_empty() {
        // Pad every word to the length of the longest one so the counts line up.
        let width = stats
            .top_words
            .iter()
            .map(|(word, _)| word.chars().count())
            .max()
            .unwrap_or(0);
        let label = if stats.top_words.len() == 1 {
            "word"
        } else {
            "words"
        };

        lines.push(String::new());
        lines.push(format!("Top {} {label}:", stats.top_words.len()));
        for (index, (word, count)) in stats.top_words.iter().enumerate() {
            lines.push(format!("  {:>2}. {word:<width$}  {count}", index + 1));
        }
    }

    lines.join("\n") + "\n"
}

// ----------------------------------------------------------
// Prints text to standard output.
//
// `println!` panics if the output pipe is closed early, for example
// in `cli_word_counter file.txt | head -n 1`. Writing by hand lets us
// treat that case as normal.
// ----------------------------------------------------------
fn print_output(text: &str) {
    match io::stdout().write_all(text.as_bytes()) {
        // A closed pipe is fine; any other write error is reported.
        Err(error) if error.kind() != io::ErrorKind::BrokenPipe => {
            eprintln!("Error: cannot write output: {error}");
            process::exit(1);
        }
        _ => {}
    }
}

// ----------------------------------------------------------
// AUTOMATED TESTS
//
// Run them with: cargo test
//
// The `#[cfg(test)]` block is only compiled during `cargo test`.
// ----------------------------------------------------------
#[cfg(test)]
mod tests {
    // `super::*` imports every item from the parent module (this file).
    use super::*;

    // Small helper: turns &["a", "b"] into a Vec<String>.
    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    // ---------- normalize ----------

    #[test]
    fn test_normalize_strips_punctuation_and_lowercases() {
        assert_eq!(normalize("Hello,"), Some("hello".to_string()));
        assert_eq!(normalize("(Rust)"), Some("rust".to_string()));
        assert_eq!(normalize("'quoted'"), Some("quoted".to_string()));
    }

    #[test]
    fn test_normalize_keeps_inner_punctuation() {
        assert_eq!(normalize("don't"), Some("don't".to_string()));
        assert_eq!(normalize("e-mail"), Some("e-mail".to_string()));
    }

    #[test]
    fn test_normalize_returns_none_for_punctuation_only() {
        assert_eq!(normalize("-"), None);
        assert_eq!(normalize("..."), None);
        assert_eq!(normalize("—"), None);
    }

    // ---------- analyze ----------

    #[test]
    fn test_analyze_counts_and_ranks_words() {
        let stats = analyze("The cat and the hat. The end!", 2);

        assert_eq!(stats.lines, 1);
        assert_eq!(stats.words, 7);
        assert_eq!(stats.unique_words, 5);
        // "the" appears 3 times; the next words tie at 1, so alphabetical order wins.
        assert_eq!(
            stats.top_words,
            vec![("the".to_string(), 3), ("and".to_string(), 1)]
        );
    }

    #[test]
    fn test_analyze_is_case_insensitive() {
        let stats = analyze("Rust rust RUST,", 5);

        assert_eq!(stats.unique_words, 1);
        assert_eq!(stats.top_words, vec![("rust".to_string(), 3)]);
    }

    #[test]
    fn test_analyze_empty_text() {
        let stats = analyze("", 5);

        assert_eq!(
            stats,
            Stats {
                lines: 0,
                words: 0,
                characters: 0,
                unique_words: 0,
                top_words: vec![],
            }
        );
    }

    #[test]
    fn test_analyze_ignores_punctuation_only_tokens() {
        let stats = analyze("- — ... hello", 5);

        assert_eq!(stats.words, 1);
        assert_eq!(stats.top_words, vec![("hello".to_string(), 1)]);
    }

    #[test]
    fn test_analyze_counts_lines() {
        assert_eq!(analyze("a\nb\n\nc", 1).lines, 4);
        assert_eq!(analyze("a\n", 1).lines, 1);
        assert_eq!(analyze("a\r\nb\r\n", 1).lines, 2);
    }

    #[test]
    fn test_analyze_counts_characters_not_bytes() {
        // "héllo" is 5 characters but 6 bytes in UTF-8.
        assert_eq!(analyze("héllo", 1).characters, 5);
    }

    #[test]
    fn test_analyze_handles_unicode_words() {
        let stats = analyze("Ünï Café café", 5);

        assert_eq!(stats.words, 3);
        assert_eq!(stats.unique_words, 2);
        assert_eq!(stats.top_words[0], ("café".to_string(), 2));
    }

    #[test]
    fn test_analyze_top_zero_and_top_larger_than_unique() {
        let none = analyze("a b", 0);
        assert_eq!(none.words, 2);
        assert!(none.top_words.is_empty());

        let all = analyze("a b", 100);
        assert_eq!(all.top_words.len(), 2);
    }

    // ---------- format_report ----------

    #[test]
    fn test_format_report_with_words() {
        let stats = analyze("the cat and the hat", 2);
        let report = format_report("test.txt", &stats);

        let expected = "\
Source:       test.txt
Lines:        1
Words:        5
Characters:   19
Unique words: 4

Top 2 words:
   1. the  2
   2. and  1
";
        assert_eq!(report, expected);
    }

    #[test]
    fn test_format_report_single_word_label() {
        let report = format_report("x", &analyze("hello", 5));
        assert!(report.contains("Top 1 word:\n"));
    }

    #[test]
    fn test_format_report_without_words() {
        let report = format_report("<stdin>", &analyze("  \n ", 5));

        assert!(report.contains("Words:        0"));
        assert!(report.contains("No words found."));
        assert!(!report.contains("Top"));
    }

    #[test]
    fn test_format_report_top_zero_hides_ranking() {
        let report = format_report("x", &analyze("hello world", 0));
        assert!(!report.contains("Top"));
        assert!(!report.contains("No words found."));
    }

    // ---------- parse_args ----------

    #[test]
    fn test_parse_args_defaults() {
        let config = parse_args(&args(&[])).unwrap();

        assert_eq!(
            config,
            Config {
                path: None,
                top: DEFAULT_TOP,
                show_help: false,
            }
        );
    }

    #[test]
    fn test_parse_args_file_and_top() {
        let config = parse_args(&args(&["notes.txt", "--top", "3"])).unwrap();
        assert_eq!(config.path, Some("notes.txt".to_string()));
        assert_eq!(config.top, 3);

        // The option can also come before the file, in short form.
        let config = parse_args(&args(&["-n", "10", "notes.txt"])).unwrap();
        assert_eq!(config.path, Some("notes.txt".to_string()));
        assert_eq!(config.top, 10);
    }

    #[test]
    fn test_parse_args_help() {
        assert!(parse_args(&args(&["-h"])).unwrap().show_help);
        assert!(parse_args(&args(&["--help"])).unwrap().show_help);
    }

    #[test]
    fn test_parse_args_dash_means_stdin_path() {
        let config = parse_args(&args(&["-"])).unwrap();
        assert_eq!(config.path, Some("-".to_string()));
    }

    #[test]
    fn test_parse_args_errors() {
        assert_eq!(
            parse_args(&args(&["--top"])),
            Err("missing value for --top".to_string())
        );
        assert_eq!(
            parse_args(&args(&["-n", "abc"])),
            Err("invalid number for -n: 'abc'".to_string())
        );
        assert_eq!(
            parse_args(&args(&["--top", "-3"])),
            Err("invalid number for --top: '-3'".to_string())
        );
        assert_eq!(
            parse_args(&args(&["--bogus"])),
            Err("unknown option: --bogus".to_string())
        );
        assert_eq!(
            parse_args(&args(&["a.txt", "b.txt"])),
            Err("only one input file is supported".to_string())
        );
    }

    // ---------- read_input ----------

    #[test]
    fn test_read_input_missing_file_returns_error() {
        let result = read_input(Some("this_file_does_not_exist.txt"));

        let message = result.unwrap_err();
        assert!(message.starts_with("cannot read 'this_file_does_not_exist.txt'"));
    }
}
