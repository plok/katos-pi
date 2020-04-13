use std::fmt;
use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    let dirs = [
        ("/var/cache/pacman/pkg", 0755),
        ("/var/lib/pacman", 0755),
        ("/var/log", 0755),
        ("/dev", 0755),
        ("/run", 0755),
        ("/etc/pacman.d", 0755),
        ("/tmp", 1777),
        ("/sys", 0555),
        ("/proc", 0555),
    ];

    // TODO check if given argument is a directory

    for dir in &dirs {
        let output = Command::new("mkdir")
        .arg("--parents")
        .arg(format!("--mode={}", dir.1))
        .arg(format!("{}/{}/", args[1], dir.0))
        .output().unwrap_or_else(|e| {
            panic!("failed to create directory: {}", dir.0)
        });

        if !output.status.success() {
            print!("failed to create directory: {}", dir.0)
        }
    }

    
}
