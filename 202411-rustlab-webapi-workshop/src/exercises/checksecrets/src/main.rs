use clap::Parser;
use regex::Regex;

#[derive(Parser)]
struct Args {
    // TODO: add field for the path of the file to check

    /// List of regex patterns to match on
    #[arg(long)]
    patterns: Vec<String>,

    /// A path to a file that contains a list of known passwords
    /// Also known as a "rainbow table".
    #[arg(long)]
    known_passwords_file: Option<String>,
}

#[derive(Debug)]
struct SecretsReport {
    /// File path
    path: String,

    /// A list of line numbers that contain failed checks
    lines: Vec<u32>,
}

fn read_file(path: &str) -> std::io::Result<String> {
    todo!()
}

fn check_for_secrets(path: &str, text: &str) -> SecretsReport {
    todo!()
}

fn main() {
    let path = todo!();
    let text = read_file(path);
    let report = check_for_secrets(path, text);

    if !report.lines.is_empty() {
        println!("{report:#?}")
    }
}

// Bonus task: use the walkdir crate to scan entire directories
// Bonus task: add a score for each hit
// Bonus task: inspect the files' metadata to std::fs module to
