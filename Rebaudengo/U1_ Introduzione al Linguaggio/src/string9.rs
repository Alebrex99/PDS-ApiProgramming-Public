use std::path::Path;

fn main() {
    let percorso_str = "/cartella/file.txt";
    let percorso = Path::new(percorso_str);

    // Ora puoi utilizzare il percorso
    println!("Il percorso è: {:?}", percorso);
}