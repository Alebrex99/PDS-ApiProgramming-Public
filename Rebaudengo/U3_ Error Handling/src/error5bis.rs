use std::error::Error;

fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        return Err(String::from("Divisione per zero")) ;
    }
    Ok(x / y)
}

fn main() -> Result<(), Box<dyn Error>> {
    let result = divide(10.0, 5.0)? ;
    println!("Il risultato della divisione è: {}", result);

    let text = "42";
    let number: i32 = text.parse()?;
    println!("Il numero è: {}", number);

    Ok(())
}
