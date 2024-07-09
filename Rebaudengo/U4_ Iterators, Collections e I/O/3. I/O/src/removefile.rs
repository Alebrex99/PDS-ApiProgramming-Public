use std::fs;

fn main() -> std::io::Result<()> {
    // Definisci il percorso del file da rimuovere
    let file_to_remove = "./new.txt";

    // Rimuovi il file
    fs::remove_file(file_to_remove)?;

    println!("File rimosso con successo!");

    Ok(())
}
