use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Apro il file in modalità lettura
    let mut file = File::open("test.txt")?;

    // Dichiaro un array vuoto per contenere i byte letti
    let mut buffer = [0; 5]; // Leggo 5 byte

    // Leggo esattamente 5 byte dal file
    file.read_exact(&mut buffer)?;

    // Stampo i byte letti
    println!("I byte letti sono: {:?}", buffer);

    Ok(())
}

