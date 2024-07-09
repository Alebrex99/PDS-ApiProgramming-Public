fn divide(x: f64, y: f64) -> Result<f64, &'static str> {
    if y == 0.0 {
        Err("Impossibile dividere per zero")
    } else {
        Ok(x / y)
    }
}

fn main() {
    let dividend = 10.0;
    let divisor_success = 2.0;
    let divisor_error = 0.0;

    let result_success = divide(dividend, divisor_success);
    let result_error = divide(dividend, divisor_error);

    println!("10 / 2");
    match result_success.ok() {
        Some(value) => println!("Il risultato della divisione è: {}", value),
        None => println!("Nessun risultato presente"),
    }
    match result_success.err() {
        Some(message) => println!("Errore: {}", message),
        None => println!("Nessun errore presente"),
    }
    println!("10 / 0");
    match result_error.ok() {
        Some(value) => println!("Il risultato della divisione è: {}", value),
        None => println!("Nessun risultato presente"),
    }
    match result_error.err() {
        Some(error_message) => println!("Errore: {}", error_message),
        None => println!("Nessun errore presente"),
    }
}