use std::process::Command;
use chrono::Local;
use std::env;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().skip(1).collect();

    // Check for version flag
    if args.len() == 1 && (args[0] == "--version" || args[0] == "-v") {
        println!("bezan v1.2");
        return;
    }

    // Get the current date in MM/DD/YYYY format
    let date = Local::now().format("%m/%d/%Y").to_string();
    
    // Separate files from message
    let (files, message) = parse_arguments(&args, &date);

    // Execute git add with the specified files
    let mut add_cmd = Command::new("git");
    add_cmd.arg("add");
    for file in &files {
        add_cmd.arg(file);
    }
    
    let add_output = add_cmd.output().expect("Failed to execute git add");

    if !add_output.status.success() {
        eprintln!("Error in git add: {}", String::from_utf8_lossy(&add_output.stderr));
        std::process::exit(1);
    }

    // Execute git commit -m "<message>"
    let commit_output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&message)
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

fn parse_arguments(args: &[String], date: &str) -> (Vec<String>, String) {
    if args.is_empty() {
        // No arguments: add all files, use date as message
        return (vec![".".to_string()], date.to_string());
    }

    // Check if the last argument is a message (doesn't look like a file)
    let last_arg = &args[args.len() - 1];
    let is_message = !std::path::Path::new(last_arg).exists() && !last_arg.contains('.');

    if args.len() == 1 {
        if is_message {
            // Single argument that's a message: add all files, use custom message
            (vec![".".to_string()], last_arg.clone())
        } else {
            // Single argument that's a file: add that file, use date
            (vec![last_arg.clone()], date.to_string())
        }
    } else {
        // Multiple arguments
        if is_message {
            // Last arg is message, everything else is files
            let files = args[..args.len() - 1].to_vec();
            (files, last_arg.clone())
        } else {
            // All args are files, use date as message
            (args.to_vec(), date.to_string())
        }
    }
}
