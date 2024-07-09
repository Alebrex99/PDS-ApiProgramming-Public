use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Apro due file in modalità lettura
    let file1 = File::open("file1.txt")?;
    let file2 = File::open("file2.txt")?;

    // Concateno i due lettori
    let mut chained_reader = file1.chain(file2);

    // Dichiaro un buffer per contenere i dati letti
    let mut buffer = Vec::new();

    // Leggo i dati concatenati nei buffer
    chained_reader.read_to_end(&mut buffer)?;

    // Converto il buffer in una stringa UTF-8 e la stampo
    match String::from_utf8(buffer) {
        Ok(content) => println!("{}", content),
        Err(_) => println!("Errore nella decodifica del contenuto"),
    }

    Ok(())
}

