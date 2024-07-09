use std::process::{Command};
use std::thread;
use std::time::Duration;

fn main() {
    // Creazione di un processo figlio
    let mut child = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("failed to start child process");

    // Simulare un ritardo nel processo genitore
    println!("Waiting for the child process to finish...");
    thread::sleep(Duration::from_secs(7));

    // Attesa del processo figlio per evitare che diventi zombie
    let status = child.wait()
        .expect("failed to wait on child process");

    println!("Child process exited with status: {}", status);
}