use std::env;
use std::fs::OpenOptions;
use filetime::{set_file_times, FileTime};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let now = FileTime::now();

    for path in &args {
        if OpenOptions::new().create(true).write(true).open(path).is_err() {
            println!("error creating {path}");
            continue;
        }

        if let Err(e) = set_file_times(path, now, now) {
            println!("error setting time on {path}: {e}");
        }
    }
}