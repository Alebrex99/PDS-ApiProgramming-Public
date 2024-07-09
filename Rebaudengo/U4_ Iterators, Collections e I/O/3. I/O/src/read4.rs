

use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Apro il file in modalità lettura
    let mut file = File::open("test.txt")?;

    // Ottengo un iteratore sui byte del file
    let bytes_iter = file.bytes();

    // Itero sui byte e stampo il valore di ciascun byte
    for byte in bytes_iter {
        match byte {
            Ok(b) => println!("Byte: {}", b),
            Err(e) => println!("Errore durante la lettura del byte: {}", e),
        }
    }

    Ok(())
}
