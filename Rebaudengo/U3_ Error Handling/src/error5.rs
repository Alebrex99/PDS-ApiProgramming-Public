fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        return Err(String::from("Divisione per zero")) ;
    }
    Ok(x / y)
}

fn main() -> Result<(), String> {
    let result = divide(10.0, 5.0)? ;
    println!("Il risultato della divisione è: {}", result);

    let result2 = divide(10.0, 0.0)? ;
    println!("Il risultato della divisione è: {}", result2);

    Ok(())
}

