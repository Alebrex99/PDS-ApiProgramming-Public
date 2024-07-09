use std::io::SeekFrom;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Seek;
use std::io::Read;
use std::io;

fn main() -> io::Result<()> {
    // Apro il file in modalità di scrittura e lettura
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("example.txt")?;
    file.write_all(b"Hello, world!")?;
    // Sposto il cursore di lettura/scrittura alla fine del file
    file.seek(SeekFrom::End(0))?;

    // Scrivo dei dati aggiuntivi alla fine del file
    file.write_all(b" Additional data")?;

    // Sposto il cursore di lettura/scrittura alla posizione 7 nel file
    file.seek(SeekFrom::Start(7))?;

    // Scrivo dei dati in una posizione specifica nel file
    file.write_all(b"Rust ")?;

    // Sposto il cursore di lettura/scrittura all'inizio del file
    file.seek(SeekFrom::Start(0))?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    println!("Contenuto del file: {}", buffer);
    Ok(())
}
