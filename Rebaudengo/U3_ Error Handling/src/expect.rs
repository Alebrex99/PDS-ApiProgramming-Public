fn divide(x: f64, y: f64) -> Result<f64, &'static str> {
    if y == 0.0 {
        Err("Impossibile dividere per zero")
    } else {
        Ok(x / y)
    }
}

fn main() {
    let dividend = 10.0;
    let divisor = 0.0;

    let result = divide(dividend, divisor);

    // Utilizzando expect() per ottenere il valore o stampare un messaggio di errore personalizzato
    let quotient = result.expect("ATTENZIONE, ERRORE NEI DATI");

    println!("Il risultato della divisione è: {}", quotient);
}