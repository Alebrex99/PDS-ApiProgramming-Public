
use std::fs::File;
use std::io::Read;

fn main() {
    // Apro il file in modalità lettura
    let mut file = match File::open("test.txt") {
        Ok(file) => file,
        Err(e) => {
            println!("Errore durante l'apertura del file: {}", e);
            return;
        }
    };

    // Creo un buffer vuoto per contenere i dati letti
    let mut buffer = Vec::new();

    // Leggo il contenuto del file nel buffer
    match file.read_to_end(&mut buffer) {
        Ok(_) => {
            // Converto il buffer in una stringa UTF-8 e stampo il contenuto
            match String::from_utf8(buffer) {
                Ok(content) => println!("{}", content),
                Err(_) => println!("Errore nella decodifica del contenuto del file"),
            }
        }
        Err(e) => println!("Errore durante la lettura del file: {}", e),
    }
}