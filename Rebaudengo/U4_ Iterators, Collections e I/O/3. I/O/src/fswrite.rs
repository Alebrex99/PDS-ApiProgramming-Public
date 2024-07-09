use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let file_path = "./file.txt";
    let text = "Questo è un testo scritto con fs::write!";
    fs::write(file_path, text)?;

    Ok(())
}
