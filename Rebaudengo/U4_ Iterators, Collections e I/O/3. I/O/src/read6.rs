

use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Apro il file in modalità lettura
    let file = File::open("test.txt")?;

    // Creo un nuovo lettore che legge solo i primi 10 byte dal file originale
    let mut limited_reader = file.take(8);

    // Dichiaro un buffer per contenere i dati letti
    let mut buffer = Vec::new();

    // Leggo i primi 10 byte dal file e li memorizzo nel buffer
    limited_reader.read_to_end(&mut buffer)?;

    // Converto il buffer in una stringa UTF-8 e la stampo
    match String::from_utf8(buffer) {
        Ok(content) => println!("{}", content),
        Err(_) => println!("Errore nella decodifica del contenuto"),
    }

    Ok(())
}
