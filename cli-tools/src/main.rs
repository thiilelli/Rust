use std::process::Command;

fn main() {
    let status = Command::new("rustc")
        .arg("--version")
        .status()
        .expect("failed to run command");

    if status.success() {
        println!("✔ Command succeeded");
    } else {
        println!("✘ Command failed");
    }
}

use clap::Parser;

/// Rust version of `echo` supporting -n to omit the trailing newline.
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Input text to print
    #[arg(required = true)]
    text: Vec<String>,

    /// Do not print the trailing newline
    #[arg(short = 'n')]
    omit_newline: bool,
}

fn main() {
    // Parse command-line arguments into the Args struct
    let args = Args::parse();

    // Join all given words with spaces, like `echo`
    // If -n is present, do not print a final newline
    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" }
    );
}
