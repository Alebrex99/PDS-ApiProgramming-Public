use std::io;
use std::fs::File;
use std::io::Read;

fn read_file_contents(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?; // Apre il file e gestisce gli errori con l'operatore ?
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // Legge il contenuto del file e gestisce gli errori con l'operatore ?
    Ok(contents) // Restituisce il contenuto del file se tutto è andato bene
}

fn main() {
    match read_file_contents("prova.txt") {
        Ok(contents) => println!("Contenuto del file: {}", contents),
        Err(err) => println!("Errore: {}", err),
    }
}