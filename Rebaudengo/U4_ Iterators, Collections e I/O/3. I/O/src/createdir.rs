use std::fs;

fn main() -> std::io::Result<()> {
    // Definisci il percorso della nuova directory da creare
    let new_directory_path = "./mynewdir";

    // Crea la nuova directory
    fs::create_dir(new_directory_path)?;

    println!("Directory creata con successo!");

    Ok(())
}
