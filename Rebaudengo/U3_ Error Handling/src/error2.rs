fn parse_integer(text: &str) -> Result<i32, std::num::ParseIntError> {
    // L'operatore ? gestisce la possibile generazione di un errore durante il parsing della stringa
    let number: i32 = text.parse()?;

    Ok(number)
}

fn main() {
    match parse_integer("42") {
        Ok(number) => println!("Parsed integer: {}", number),
        Err(err) => eprintln!("Error: {}", err),
    }
}