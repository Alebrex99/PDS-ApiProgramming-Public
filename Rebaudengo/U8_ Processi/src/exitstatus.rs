use std::process::Command;

fn main() {
    // Esegui un comando che si sa restituirà un valore di uscita specifico
    let command = Command::new("ls")
        .arg("/nonexistent_directory")
        .spawn();

    match command {
        Ok(mut child) => {
            // Attendere il completamento del processo figlio e ottenere l'ExitStatus
            match child.wait() {
                Ok(exit_status) => {
                    // Verificare se il processo è terminato correttamente o con un errore
                    if exit_status.success() {
                        println!("Il processo è terminato correttamente.");
                    } else {
                        println!("Il processo è terminato con errore.");
                    }

                    // Ottenere il codice di uscita del processo se disponibile
                    if let Some(code) = exit_status.code() {
                        println!("Codice di uscita del processo: {}", code);
                    } else {
                        println!("Il processo è stato terminato da un segnale.");
                    }
                }
                Err(e) => {
                    eprintln!("Errore durante l'attesa del processo: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Errore durante l'avvio del processo: {}", e);
        }
    }
}
