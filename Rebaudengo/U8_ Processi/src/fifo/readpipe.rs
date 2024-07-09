
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let pipe_path = "/tmp/my_pipe";

    // Check if the named pipe exists
    if !Path::new(pipe_path).exists() {
        eprintln!("Named pipe {} does not exist. Please create it first using mkfifo.", pipe_path);
        return;
    }

    // Open the named pipe for reading
    let pipe = match OpenOptions::new().read(true).open(pipe_path) {
        Ok(pipe) => pipe,
        Err(e) => {
            eprintln!("Failed to open named pipe: {}", e);
            return;
        }
    };

    // Create a buffered reader for the pipe
    let reader = BufReader::new(pipe);

    // Read from the pipe line by line
    for line in reader.lines() {
        match line {
            Ok(content) => println!("Received: {}", content),
            Err(e) => eprintln!("Failed to read from named pipe: {}", e),
        }
    }
}
