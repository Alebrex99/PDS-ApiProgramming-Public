use std::io::{ErrorKind, Read};
use std::fs::File;

fn main() {
    // Tentativo di apertura di un file
    match File::open("testo.txt") {
        Ok(mut file) => {
            let mut contenuto = String::new();
            // Tentativo di lettura del contenuto del file
            match file.read_to_string(&mut contenuto) {
                Ok(_) => println!("Contenuto del file: {}", contenuto),
                Err(e) => match e.kind() {
                    // Gestione specifica per diversi tipi di errori
                    ErrorKind::NotFound => println!("Il file non è stato trovato."),
                    ErrorKind::PermissionDenied => println!("Permesso negato."),
                    _ => println!("Si è verificato un errore durante la lettura del file: {}", e),
                },
            }
        },
        Err(e) => match e.kind() {
            ErrorKind::NotFound => println!("Il file non è stato trovato."),
            ErrorKind::PermissionDenied => println!("Permesso negato."),
            _ => println!("Si è verificato un errore durante l'apertura del file: {}", e),
        },
    }
}


