use std::fs;

fn main() -> std::io::Result<()> {
    // Ottieni il percorso della directory
    let directory_path = ".";

    // Leggi il contenuto della directory
    let entries = fs::read_dir(directory_path)?;

    // Itera sugli elementi nella directory
    for entry in entries {
        // Gestisci eventuali errori nell'accesso ai file/directory
        let entry = entry?;

        // Ottieni il nome dell'elemento
        let file_name = entry.file_name();

        // Stampa il nome dell'elemento
        println!("{:?}", file_name);
    }

    Ok(())
}