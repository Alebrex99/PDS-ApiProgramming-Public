use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    loop {
        println!("Enter the name of the file to open:");

        let mut filename = String::new();
        io::stdin().read_line(&mut filename);

        // Rimuove eventuali spazi vuoti, a capo o altri caratteri di controllo dal nome del file
        let filename = filename.trim();

        match File::open(filename) {
            Ok(file) => {
                println!("Successfully opened {}", filename);
                // Esegui altre operazioni sul file aperto qui se necessario
                break; // Esci dal ciclo se l'apertura ha avuto successo
            },
            Err(err) => {
                println!("Error opening {}: {}", filename, err);
                println!("Please try again.");
            }
        }
    }
}