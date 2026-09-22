use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("usage: rkill [-9] <pid|name> [pid2|name2 ...]");
        return;
    }

    let mut signal = Signal::SIGTERM;
    let mut pids: Vec<&String> = Vec::new();

    for arg in &args {
        if let Some(flag) = arg.strip_prefix('-') {
            match flag {
                "9" => signal = Signal::SIGKILL,
                "15" => signal = Signal::SIGTERM,
                other => println!("unknown flag: -{other}"),
            }
        } else {
            pids.push(arg);
        }
    }

    for target in pids {
        if let Ok(pid_num) = target.parse::<i32>() {
            send(pid_num, signal, target);
            continue;
        }

        let matches = find_pids_by_name(target);
        if matches.is_empty() {
            println!("rkill: {target}: no such process");
            continue;
        }
        for pid_num in matches {
            send(pid_num, signal, target);
        }
    }
}

fn send(pid_num: i32, signal: Signal, label: &str) {
    if let Err(e) = kill(Pid::from_raw(pid_num), signal) {
        println!("rkill: {label} ({pid_num}): {e}");
    }
}

fn find_pids_by_name(name: &str) -> Vec<i32> {
    let mut matches = Vec::new();

    let entries = match fs::read_dir("/proc") {
        Ok(e) => e,
        Err(_) => return matches,
    };

    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let Some(pid_num) = file_name.to_str().and_then(|s| s.parse::<i32>().ok()) else {
            continue; // not a PID directory (e.g. /proc/cpuinfo)
        };

        let comm_path = entry.path().join("comm");
        if let Ok(comm) = fs::read_to_string(comm_path) {
            if comm.trim() == name {
                matches.push(pid_num);
            }
        }
    }

    matches
}