use std::process::{Command, Stdio};

fn main() {
        // Esegue un comando esterno
        let mut child = Command::new("sleep")
            .arg("10")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start command");

        // Ottiene l'ID del processo
        let pid = child.id();

        // Termina il processo
        match child.kill() {
                Ok(_) => println!("Process {} killed", pid),
                Err(err) => println!("Error killing process {}: {}", pid, err),
        }
}

