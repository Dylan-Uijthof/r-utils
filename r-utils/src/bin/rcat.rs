use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("usage: rcat <file>");
        return;
    }

    for path in &args {
        match fs::read_to_string(path) {
            Ok(contents) => print!("{contents}"),
            Err(e) => println!("rcat: {path}: {e}")
        }
    }
}