use std::io::SeekFrom;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Seek;
use std::io::Read;
use std::io;
fn main() -> io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("example.txt")?;
    file.write_all(b"Prova di Testo del File")?;
    file.seek(SeekFrom::Start(0))?;         // Sposto il cursore all'inizio del file utilizzando seek_from_start
    println!("Posizione corrente del cursore: {}", file.stream_position()?);
    let mut buffer = [0; 10];
    file.read_exact(&mut buffer)?;         // Leggo i primi 10 byte dal file
    println!("I primi 10 byte del file: {:?}", buffer);
    println!("Posizione corrente del cursore: {}", file.stream_position()?);
    file.seek(SeekFrom::End(0))?;         // Sposto il cursore alla fine del file utilizzando seek_from_end
    println!("Alla Fine: posizione corrente del cursore: {}", file.stream_position()?);
    file.seek(SeekFrom::Current(-5))?;    // Sposto il cursore 5 byte indietro dalla fine del file
    println!("Indietro di 5: posizione corrente del cursore: {}", file.stream_position()?);
    file.write_all(b" Additional data")?; // Scrivo ulteriori dati alla fine del file
    file.seek(SeekFrom::Start(10))?;   // Riporto il cursore alla posizione 10 all'interno del file
    println!("Vado in posizione 10: posizione corrente del cursore: {}", file.stream_position()?);
    let mut buffer = [0; 5];
    file.read_exact(&mut buffer)?;       // Leggo 5 byte dalla posizione corrente
    println!("Dati letti dalla posizione corrente: {:?}", buffer);
    file.seek(SeekFrom::Start(0))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    println!("Tutto il file: {:?}", buffer);
    Ok(())
}

