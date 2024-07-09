use std::process::{Command, Stdio};

fn main() {
    let child_process = Command::new("ls")
            .arg("-l")
            .arg("-a")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Impossibile avviare il processo 'ls'");

    let output = child_process.wait_with_output().expect("Impossibile ottenere l'output del processo");

    if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("Output del processo 'ls':\n{}", stdout);
      } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("Errore durante l'esecuzione del processo 'ls': {}", stderr);
       }
}
