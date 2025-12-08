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
