use std::convert::TryFrom;
struct EvenNumber(i32);
impl TryFrom<i32> for EvenNumber {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err("il numero non è pari.")
        }
    }
}
fn verifica (value: i32, result: Result<EvenNumber, &str>)
{   match result {
    Ok(even) => println!("Numero pari: {}", even.0),
    Err(err) => println!("Errore: ho ricevuto {}, ma {}", value, err),}
}
fn main() {
    let even_number = 42;
    let result = EvenNumber::try_from(even_number);
    verifica(even_number, result);
    let odd_number = 37;
    let result_odd = EvenNumber::try_from(odd_number);
    verifica(odd_number, result_odd);
}
