use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn main() -> Result<(), Error> {
    // Definisci il percorso del nuovo file da creare
    let file_path = "./new_file.txt";

    // Crea un nuovo file
    let mut file = File::create(file_path)?;

    // Scrivi una stringa direttamente nel file
    let text = "Questo è un nuovo file creato in Rust!";
    file.write(text.as_bytes())?;

    println!("File creato e scritto con successo!");

    Ok(())
}
