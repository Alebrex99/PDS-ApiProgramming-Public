use std::io::BufReader;
use std::fs::File;
use std::io::Error;
use std::io::BufRead;




fn main() -> Result<(), Error> {
    let file = File::open("./prova.txt")?;

    // Creare un BufReader per il file
    let mut buf_reader = BufReader::new(file);

    // Buffer per memorizzare il contenuto letto
    let mut buffer = String::new();
    loop {
        // Riempire il buffer interno di BufReader
        buf_reader.fill_buf()?;

        // Se il buffer è vuoto, siamo alla fine del file, quindi usciamo dal ciclo
        if buf_reader.buffer().is_empty() { break; }

        // Consuma i primi 5 byte dei dati letti dal buffer
        buf_reader.consume(5);

        // Leggi il prossimo blocco di dati dal buffer
        buf_reader.read_line(&mut buffer)?;

        // Stampa il blocco di dati letti
        println!("{}", buffer);
        // Pulisci il buffer per prepararlo per la prossima lettura
        buffer.clear();
    }
    Ok(())
}
