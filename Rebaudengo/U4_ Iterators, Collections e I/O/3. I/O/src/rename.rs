use std::fs;

fn main() -> std::io::Result<()> {
    // Definisci il percorso del file o della directory da rinominare
    let old_path = "./file.txt";

    // Definisci il nuovo nome o percorso del file o della directory
    let new_path = "./new.txt";

    // Rinomina il file o la directory
    fs::rename(old_path, new_path)?;

    println!("Rinominato con successo!");

    Ok(())
}