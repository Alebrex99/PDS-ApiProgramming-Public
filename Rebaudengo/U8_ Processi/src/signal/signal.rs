use std::process::{Command, Stdio};
use std::os::unix::process::ExitStatusExt;
// ExitStatusExt is an extension trait for extracting any such signal, and other details, from the ExitStatus.
fn main() {
    // Esempio: avvia un processo figlio
    let mut child_process =  Command::new("timer")
        .env("PATH", "./target/debug/")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let child_pid = child_process.id(); // Ottieni l'ID del processo figlio
    let result = Command::new("kill") // Invia il segnale SIGTERM al processo figlio
        .arg("-15") // Codice del segnale SIGTERM
        .arg(child_pid.to_string())
        .status();
    match result {
        Ok(status) => {
            if status.success() {
                println!("Segnale SIGTERM inviato al processo figlio.");
            } else {
                println!("Errore nell'invio del segnale.");
            }
        }
        Err(_) => { println!("Errore nell'esecuzione del comando kill."); }
    }
    match child_process.wait() {
        Ok(exit_status) => {
            // Verificare se il processo è terminato correttamente o con un errore
            if exit_status.success() {
                println!("Il processo è terminato correttamente.");
            } else {
                println!("Il processo è terminato con errore: {:?}",exit_status);
            }
            // Ottenere il codice di uscita del processo se disponibile
            if let Some(code) = exit_status.code() {
                println!("Codice di uscita del processo: {}", code);
            } else {
                // Utilizza ExitStatusExt per ottenere informazioni
                if let Some(signal) = exit_status.signal() {
                    println!("Processo terminato dal segnale: {}", signal);
                }
                if exit_status.core_dumped() {
                    println!("Il processo ha generato un core dump");
                }
            }
        }
        Err(e) => { eprintln!("Errore durante l'attesa del processo: {}", e); }
    }
}
