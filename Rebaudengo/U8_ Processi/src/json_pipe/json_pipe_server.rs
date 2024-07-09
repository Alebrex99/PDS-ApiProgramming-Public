use serde::{Serialize, Deserialize};
use std::io::Read;
use std::process::{Command, Stdio};

#[derive(Serialize, Deserialize, Debug)]
struct Point {x: i32, y: i32}

fn main() {
    let mut child = Command::new("client") // Avvia il processo figlio (client)
        .env("PATH", "./target/debug/")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start child process");

    let mut pipe_reader = child.stdout.take().expect("Failed to open stdout");

    let mut s = String::new();
    match pipe_reader.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read client stdout: {}", why),
        Ok(_) => print!("Client responded with:\n{}", s),
    }
    let deserialized: Point= serde_json::from_str(&s).unwrap();
    println!("message from client {}", s);
    println!("deserialized = {:?}", deserialized);

    // Attendere che il processo figlio termini
    child.wait().expect("Failed to wait for child process");
}
