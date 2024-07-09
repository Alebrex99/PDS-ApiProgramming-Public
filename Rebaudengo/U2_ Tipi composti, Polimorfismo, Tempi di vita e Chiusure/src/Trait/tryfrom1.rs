use std::convert::TryFrom;

// Definiamo una struttura PositiveInteger
struct PositiveInteger(u32);

// Implementiamo il tratto TryFrom per convertire da u32 a PositiveInteger
impl TryFrom<u32> for PositiveInteger {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(PositiveInteger(value))
        } else {
            Err("il valore deve essere maggiore di zero")
        }
    }
}

fn verifica (value: u32, result: Result<PositiveInteger, &str>)
{
    match result {
        Ok(pos_int) => println!("Valore positivo: {}", pos_int.0),
        Err(err) => println!("Errore: ho ricevuto {} {}", value, err),
    }
}

fn main() {
    let valid_value = 42;
    let result_valid = PositiveInteger::try_from(valid_value);
    verifica(valid_value, result_valid);

    let invalid_value = 0;
    let result_invalid = PositiveInteger::try_from(invalid_value);
    verifica(invalid_value, result_invalid);
}