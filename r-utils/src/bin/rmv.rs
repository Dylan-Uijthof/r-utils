use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        println!("usage: rmv <src> <dst>");
        return;
    }

    let src = Path::new(&args[0]);
    let dst = Path::new(&args[1]);

    // Fast path: same filesystem, the OS just repoints the entry.
    if fs::rename(src, dst).is_ok() {
        return;
    }

    // Fallback: different disks, rename can't span them.
    // Copy everything over, then remove the original.
    let copy_result = if src.is_dir() {
        copy_dir(src, dst)
    } else {
        fs::copy(src, dst).map(|_| ())
    };

    if let Err(e) = copy_result {
        println!("rmv: {e}");
        return;
    }

    let remove_result = if src.is_dir() {
        fs::remove_dir_all(src)
    } else {
        fs::remove_file(src)
    };

    if let Err(e) = remove_result {
        println!("rmv: copied but failed to remove original: {e}");
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