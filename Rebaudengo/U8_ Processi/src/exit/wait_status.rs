use std::process::{Command, Stdio};
use std::os::unix::process::ExitStatusExt;
fn main() {
    let mut child_process =  Command::new("exituno")
        .env("PATH", "./target/debug/")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let child_pid = child_process.id(); // Ottieni l'ID del processo figlio
    match child_process.wait() {
        Ok(exit_status) => {
            // Verificare se il processo è terminato correttamente o con un errore
            if exit_status.success() {
                println!("Il processo è terminato correttamente.{:?}",exit_status);
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
