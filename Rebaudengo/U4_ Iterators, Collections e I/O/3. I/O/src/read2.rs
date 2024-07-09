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

    // Dichiaro una stringa vuota per contenere il contenuto del file
    let mut content = String::new();

    // Leggo il contenuto del file nella stringa
    match file.read_to_string(&mut content) {
        Ok(_) => {
            // Stampo il contenuto del file
            println!("{}", content);
        }
        Err(e) => println!("Errore durante la lettura del file: {}", e),
    }
}

