use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("usage: rtail <file>");
        return;
    }

    for path in &args {
        match fs::read_to_string(path) {
            Ok(contents) => {
                let lines: Vec<&str> = contents.lines().collect();
                let start = if lines.len() > 10 { lines.len() - 10 } else { 0 };

                for line in &lines[start..] {
                    println!("{line}");
                }
            }
            Err(e) => println!("rtail: {path}: {e}"),
        }
    }
}