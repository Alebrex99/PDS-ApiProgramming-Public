use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let file_path = "./file.txt";

    // Leggi il contenuto del file in una stringa
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    // Stampare il contenuto del file
    println!("Contenuto del file:\n{}", contents);

    Ok(())
}
