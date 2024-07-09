fn divide(x: f64, y: f64) -> Result<f64, &'static str> {
    if y == 0.0 {
        Err("Impossibile dividere per zero")
    } else {
        Ok(x / y)
    }
}

fn main() {
    let dividend = 10.0;
    let divisor_success = 2.0;
    let divisor_error = 0.0;

    let result_success = divide(dividend, divisor_success).unwrap();
    println!("{}", result_success);

    let result_error = divide(dividend, divisor_error).unwrap_err();
    println!("{}", result_error);
}