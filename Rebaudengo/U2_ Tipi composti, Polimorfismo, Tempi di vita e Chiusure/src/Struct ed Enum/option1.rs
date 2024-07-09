fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    let result = divide(820.0, 15.2);

    match result {
        Some(value) => println!("La divisione è {}.", value),
        None => println!("Non è possibile eseguire la divisione."),
    }
}

