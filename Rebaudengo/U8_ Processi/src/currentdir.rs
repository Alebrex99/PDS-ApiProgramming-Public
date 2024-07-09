use std::process::Stdio;
use std::process::Command;

fn main() {
    let output = Command::new("ls")
        .current_dir("/bin")
        .stdout(Stdio::inherit())
        .output()
        .expect("ls command failed to start");
}