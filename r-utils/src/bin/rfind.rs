use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let start = if args.is_empty() { "." } else { &args[0] };

    walk(Path::new(start));
}

fn walk(dir: &Path) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            println!("rfind: {}: {e}", dir.display());
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                println!("rfind: {e}");
                continue;
            }
        };

        let path = entry.path();
        println!("{}", path.display());

        if path.is_dir() {
            walk(&path);
        }
    }
}