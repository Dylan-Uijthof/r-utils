use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut parents = false;
    let mut paths: Vec<&String> = Vec::new();

    for arg in &args {
        if let Some(flags) = arg.strip_prefix('-') {
            for c in flags.chars() {
                match c {
                    'p' => parents = true,
                    _ => println!("unknown flag: -{c}"),
                }
            }
        } else {
            paths.push(arg);
        }
    }

    if paths.is_empty() {
        println!("usage: rmkdir [-p] <dir> [dir2 ...]");
        return;
    }

    for path in paths {
        let result = if parents {
            fs::create_dir_all(path)
        } else {
            fs::create_dir(path)
        };

        if let Err(e) = result {
            println!("rmkdir: {path}: {e}");
        }
    }
}