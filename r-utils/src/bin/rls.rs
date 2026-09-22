use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let path = if args.is_empty() {"."} else {&args[0]};

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(e) => {
            println!("error reading {path}: {e}");
            return;
        }
    };

    for entry in entries {
        let entry = entry.expect("failed to read entry");
        let name = entry.file_name();
        println!("{}", name.to_string_lossy());
    }
}