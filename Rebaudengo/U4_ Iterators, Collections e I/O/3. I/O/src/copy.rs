use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Percorso del file di origine
    let source_path = "./prova.txt";

    // Percorso di destinazione per il file copiato
    let destination_path = "./file.txt";

    // Copia il file
    fs::copy(source_path, destination_path)?;

    println!("File copiato con successo!");

    Ok(())
}
