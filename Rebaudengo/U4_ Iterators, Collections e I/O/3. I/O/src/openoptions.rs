use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file_path = "./new_file.txt";

    // Aprire il file in modalità di scrittura, troncandolo se esiste già
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;

    let text = "Questo è un nuovo file creato in Rust!";
    file.write(text.as_bytes())?;


    Ok(())
}
