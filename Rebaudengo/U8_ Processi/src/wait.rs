use std::process::Command;

fn main() {
    // Crea un'istanza di Command per il comando "echo" con l'argomento "Hello World"
    let mut process = Command::new("echo")
        .arg("Hello World")
        .spawn()
        .expect("Failed to spawn command");

    // Attendere che il processo figlio termini
    let exit_status = process.wait();

    // Verificare lo stato di uscita del processo figlio
    if exit_status.expect("Errore nel processo spawned").success()
    { println!("Child process has exited successfully."); }
}