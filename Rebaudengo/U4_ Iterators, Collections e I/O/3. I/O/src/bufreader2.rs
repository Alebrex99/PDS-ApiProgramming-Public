use std::io::BufReader;
use std::fs::File;
use std::io::Error;
use std::io::BufRead;


fn main() -> Result<(), Error> {
    // Aprire il file in modalità di lettura
    let file = File::open("./prova.txt")?;

    // Creare un BufReader per il file
    let mut buf_reader = BufReader::new(file);

    // Buffer per memorizzare il contenuto letto
    let mut buffer = String::new();

    // Riempire il buffer interno di BufReader
    buf_reader.fill_buf()?;

    // Leggere il contenuto del buffer fino a quando non si raggiunge la fine del file
    while buf_reader.buffer().len() > 0 {
        // Leggere una riga dal buffer
        let bytes_read = buf_reader.read_line(&mut buffer)?;

        // Stampa la riga letta
        print!("{}", buffer);

        // Pulisci il buffer per prepararlo per la prossima lettura
        buffer.clear();
        // Continua a riempire il buffer interno di BufReader
        buf_reader.fill_buf()?;
    }
    Ok(())
}