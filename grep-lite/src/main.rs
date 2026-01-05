use std::env;
use std::fs;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure correct usage
    if args.len() != 3 {
        eprintln!("Usage: grep-lite <pattern> <filename>");
        std::process::exit(1);
    }

    let pattern = &args[1];
    let filename = &args[2];

    // Read the file contents
    let contents = fs::read_to_string(filename)
        .expect("Could not read file");

    // Search and print matching lines
    for line in contents.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}
