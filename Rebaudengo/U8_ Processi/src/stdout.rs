use std::process::Stdio;
use std::process::Command;

fn main() {

    let mut output = Command::new("echo")
        .arg("Hello, world!")
        .stdout(Stdio::null()) // This stream will be ignored.
        // equivalent of attaching the stream to /dev/null
        .output()
        .expect("Failed to execute command");
    println!("{:?}", output);

    output = Command::new("echo")
        .arg("Hello, world!")
        .stdout(Stdio::inherit()) // The child inherits from the corresponding parent descriptor
        .output()
        .expect("Failed to execute command");
    println!("{:?}", output);

    output = Command::new("echo")
        .arg("Hello, world!")
        .stdout(Stdio::piped()) //  A new pipe should be arranged to connect the parent and child processes
        .output()
        .expect("Failed to execute command");
    println!("{:?}", output);
}