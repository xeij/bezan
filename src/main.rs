use std::process::Command;
use chrono::Local;

fn main() {
    // Get the current date in MM/DD/YYYY format
    let date = Local::now().format("%m/%d/%Y").to_string();

    // Execute git add .
    let add_output = Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("Failed to execute git add");

    if !add_output.status.success() {
        eprintln!("Error in git add: {}", String::from_utf8_lossy(&add_output.stderr));
        std::process::exit(1);
    }

    // Execute git commit -m "<date>"
    let commit_output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&date)
        .output()
        .expect("Failed to execute git commit");

    if !commit_output.status.success() {
        eprintln!("Error in git commit: {}", String::from_utf8_lossy(&commit_output.stderr));
        std::process::exit(1);
    }

    // Execute git push
    let push_output = Command::new("git")
        .arg("push")
        .output()
        .expect("Failed to execute git push");

    if !push_output.status.success() {
        eprintln!("Error in git push: {}", String::from_utf8_lossy(&push_output.stderr));
        std::process::exit(1);
    }

    println!("Success");
}
