use std::fs::File;
use std::io;
use std::io::Write;
fn main() -> io::Result<()> {
    let mut file = File::create("output.txt")?;

    // Dati da scrivere nel file
    let data = b"Hello, world!\n";

    let num_writes = 5; // Numero di volte che voglio scrivere i dati nel file
    let mut total_bytes_written = 0;

    // Ciclo per scrivere più volte i dati nel file
    for _ in 0..num_writes {
        // Scrivo i dati nel file e controllo il valore di ritorno
        match file.write(data) {
            Ok(bytes_written) => {
                total_bytes_written += bytes_written; // Aggiorno il conteggio totale dei byte scritti
            }
            Err(err) => {
                // Se si verifica un errore durante la scrittura
                // stampo un messaggio di errore e termino il programma
                eprintln!("Errore durante la scrittura nel file: {}", err);
                return Err(err);
            }
        }
    }
    // Stampo il numero totale di byte scritti con successo nel file
    println!("Totale byte scritti nel file: {}", total_bytes_written);
    Ok(())
}



