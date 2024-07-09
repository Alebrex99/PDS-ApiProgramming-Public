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

    if result.is_ok() {
        println!("Il risultato della divisione è: {}", result.unwrap());
    } else {
        println!("Errore: {}", result.unwrap_err());
    }
}