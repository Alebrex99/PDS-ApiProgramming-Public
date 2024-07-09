
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn raddoppia (x: Option<f64>) -> Option<f64> {
match x {
None => None,
Some(i) => Some(i*2.0),
 }
}

fn main() {
    let result = divide(820.0, 15.2);

    let numero = raddoppia (result);
    match numero {
        Some(numero) => println!("Il risultato della divisione moltiplicato per 2 vale {}.", numero),
        None => println!("Non è possibile eseguire la divisione."),
    }
}

