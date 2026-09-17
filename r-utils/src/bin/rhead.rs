use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("usage: rhead <file>");
        return;
    }

    for path in &args {
        match fs::read_to_string(path) {
            Ok(contents) => {
                for line in contents.lines().take(10) {
                    println!("{line}");
                }
            }
            Err(e) => println!("rhead: {path}: {e}"),
        }
    }
}