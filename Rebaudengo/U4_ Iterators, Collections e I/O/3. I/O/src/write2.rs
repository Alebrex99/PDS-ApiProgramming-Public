


use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Apro il file in modalità di scrittura
    let mut file = File::create("output.txt")?;

    // Dati da scrivere nel file
    let data = b"Hello, world!\n";

    // Scrivo i dati nel buffer del file
    file.write_all(data)?;

    // Eseguo il flush e gestisco il risultato
    match file.flush() {
        Ok(()) => println!("Dati scritti con successo nel file."),
        Err(err) => {
            eprintln!("Errore durante il flushing dei dati nel file: {}", err);
            return Err(err);
        }
    }

    Ok(())
}
