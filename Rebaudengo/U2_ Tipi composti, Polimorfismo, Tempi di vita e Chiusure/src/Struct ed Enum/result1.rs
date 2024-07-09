// Defining a function that returns a Result
fn divide(x: f64, y: f64) -> Result<f64, &'static str> {
    if y == 0.0 {
        // Returning an error if division by zero is attempted
        Err("Division by zero!")
    } else {
        // Returning the result of the division
        Ok(x / y)
    }
}

fn main() {
    // Attempting division
    let dividend = 10.0;
    let divisor = 2.0;

    // Handling the result
    match divide(dividend, divisor) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}