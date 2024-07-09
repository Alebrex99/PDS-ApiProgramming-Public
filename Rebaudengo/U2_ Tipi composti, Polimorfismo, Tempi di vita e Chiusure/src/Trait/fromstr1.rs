use std::str::FromStr;

fn verifica (value: &str, result: Result<i32, ParseIntError>)
{
    match result {
        Ok(num) => println!("Numero: {}", num),
        Err(err) => println!("Errore: ho ricevuto {} {}", value, err),
    }
}

fn main() {
    // Utilizziamo il metodo parse() fornito dal tratto FromStr
    // per convertire la stringa in un numero intero
    let number_str = "42";
    let number: Result<i32, _> = number_str.parse();
    verifica(number_str, number);

    let number_str2 = "quarantadue";
    let number2: Result<i32, _> = number_str2.parse();
    verifica(number_str2, number2);
}