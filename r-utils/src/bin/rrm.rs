use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut recursive = false;
    let mut force = false;
    let mut paths: Vec<&String> = Vec::new();

    for arg in &args {
        if let Some(flags) = arg.strip_prefix('-') {
            for c in flags.chars() {
                match c {
                    'r' => recursive = true,
                    'f' => force = true,
                    _ => println!("unknown flag: -{c}"),
                }
            }
        } else {
            paths.push(arg);
        }
    }

    if paths.is_empty() {
        println!("usage: rrm [-rf] <path> [path2 ...]");
        return;
    }

    for path in paths {
        remove(path, recursive, force);
    }
}

fn remove(path: &str, recursive: bool, force: bool) {
    let p = Path::new(path);

    if !p.exists() {
        if !force {
            println!("rrm: {path}: No such file or directory");
        }
        return;
    }

    let result = if p.is_dir() {
        if recursive {
            fs::remove_dir_all(p)
        } else {
            println!("rrm: {path}: is a directory (use -r)");
            return;
        }
    } else {
        fs::remove_file(p)
    };

    if let Err(e) = result {
        if !force {
            println!("rrm: {path}: {e}");
        }
    }
}