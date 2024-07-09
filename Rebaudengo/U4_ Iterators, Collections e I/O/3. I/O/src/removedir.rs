use std::fs;

fn main() -> std::io::Result<()> {
    // Definisci il percorso della directory da rimuovere
    let directory_to_remove = "./mynewdir";

    // Rimuovi la directory
    fs::remove_dir(directory_to_remove)?;

    println!("Directory rimossa con successo!");

    Ok(())
}
