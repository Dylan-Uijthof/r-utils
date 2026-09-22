use std::env;
use std::fs;
use std::io::{self, Write, BufWriter};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let start = if args.is_empty() { "." } else { &args[0] };

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    walk(Path::new(start), &mut out);
}

fn walk(dir: &Path, out: &mut impl Write) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            let _ = writeln!(out, "rfind: {}: {e}", dir.display());
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                let _ = writeln!(out, "rfind: {e}");
                continue;
            }
        };

        let path = entry.path();
        let _ = writeln!(out, "{}", path.display());

        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
        if is_dir {
            walk(&path, out);
        }
    }
}