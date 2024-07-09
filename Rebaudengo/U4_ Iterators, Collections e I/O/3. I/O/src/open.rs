use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn main() -> Result<(), Error> {
    // Definisci il percorso del file da aprire
    let file_path = "./myfile";

    // Apri il file in modalità di lettura
    let mut file = File::open(file_path)?;

    // Leggi il contenuto del file in una stringa
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Stampa il contenuto del file
    println!("Contenuto del file:");
    println!("{}", contents);

    Ok(())
}