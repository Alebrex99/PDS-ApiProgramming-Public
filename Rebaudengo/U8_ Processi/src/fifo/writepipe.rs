use std::process::Command;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
fn main() {
    let pipe_path = "/tmp/my_pipe";

   Command::new("reader") // Avvia il processo lettore
        .env("PATH", "./target/debug/")
        .spawn()
        .expect("Failed to start child process");

    // Check if the named pipe exists
    if !Path::new(pipe_path).exists() {
        eprintln!("Named pipe {} does not exist. Please create it first using mkfifo.", pipe_path);
        return;
    }

    // Open the named pipe for writing
    let mut pipe = match OpenOptions::new().write(true).open(pipe_path) {
        Ok(pipe) => pipe,
        Err(e) => {
            eprintln!("Failed to open named pipe: {}", e);
            return;
        }
    };

    let message = "Hello from Rust!\n";

    // Write a message to the named pipe
    match pipe.write_all(message.as_bytes()) {
        Ok(_) => println!("Message sent to the named pipe."),
        Err(e) => eprintln!("Failed to write to the named pipe: {}", e),
    }
}

