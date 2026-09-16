use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("usage: rwc <file>");
        return;
    }

    for path in &args {
        match fs::read_to_string(path) {
            Ok(contents) => {
                let lines = contents.lines().count();
                let words = contents.split_whitespace().count();
                let bytes = contents.len();
                println!("l:{lines} w:{words} b:{bytes} {path}");
            }
            Err(e) => println!("rwc: {path}: {e}"),
        }
    }
}