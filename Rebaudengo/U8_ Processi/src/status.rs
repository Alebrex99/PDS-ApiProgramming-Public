use std::process::Command;

fn main() {
    // Esegui il comando ls e ottieni il suo stato di uscita
    let child = Command::new("ls")
        .status()
        .expect("failed to execute process");

    // Stampa lo stato di uscita del processo
    if child.success() {
        println!("Process exited successfully with status: {}", child);
    } else {
        println!("Process exited with failure status: {}", child);
    }
}
