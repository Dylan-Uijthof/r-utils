use std::env;
use std::fs;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("usage: rgrep <pattern> [file ...]");
        return;
    }

    let pattern = &args[0];
    let paths = &args[1..];

    if paths.is_empty() {
        // No file given: read from stdin instead, same as real grep.
        let mut contents = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut contents) {
            println!("rgrep: stdin: {e}");
            return;
        }
        print_matches(&contents, pattern, None);
        return;
    }

    let multiple_files = paths.len() > 1;

    for path in paths {
        match fs::read_to_string(path) {
            Ok(contents) => {
                let label = if multiple_files { Some(path.as_str()) } else { None };
                print_matches(&contents, pattern, label);
            }
            Err(e) => println!("rgrep: {path}: {e}"),
        }
    }
}

fn print_matches(contents: &str, pattern: &str, label: Option<&str>) {
    for line in contents.lines() {
        if line.contains(pattern) {
            match label {
                Some(path) => println!("{path}: {line}"),
                None => println!("{line}"),
            }
        }
    }
}