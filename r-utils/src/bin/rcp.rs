use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut recursive = false;
    let mut paths: Vec<&String> = Vec::new();

    for arg in &args {
        if let Some(flags) = arg.strip_prefix('-') {
            for c in flags.chars() {
                match c {
                    'r' => recursive = true,
                    _ => println!("unknown flag: -{c}"),
                }
            }
        } else {
            paths.push(arg);
        }
    }

    if paths.len() != 2 {
        println!("usage: rcp [-r] <src> <dst>");
        return;
    }

    let src = Path::new(paths[0]);
    let dst = Path::new(paths[1]);

    if src.is_dir() {
        if !recursive {
            println!("rcp: {}: is a directory (use -r)", paths[0]);
            return;
        }
        if let Err(e) = copy_dir(src, dst) {
            println!("rcp: {e}");
        }
    } else {
        if let Err(e) = fs::copy(src, dst) {
            println!("rcp: {e}");
        }
    }
}

fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let from = entry.path();
        let to = dst.join(entry.file_name());

        if from.is_dir() {
            copy_dir(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }

    Ok(())
}