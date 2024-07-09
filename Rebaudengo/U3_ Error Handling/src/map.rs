fn divide(x: f64, y: f64) -> Result<f64, &'static str> {
    if y == 0.0 {
        Err("Impossibile dividere per zero")
    } else {
        Ok(x / y)
    }
}

fn to_percentage(value: f64) -> f64 {
    value * 100.0
}

fn main() {
    let dividend = 12.0;
    let divisor = 55.0;

    // Utilizzando map() per convertire il risultato della divisione in percentuale
    let result = divide(dividend, divisor).map(to_percentage);

    match result {
        Ok(value) => println!("Il risultato della divisione in percentuale è: {:.1}%", value),
        Err(error_message) => println!("Errore: {}", error_message),
    }
}