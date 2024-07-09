use std::str::FromStr;

fn main() {
    let string_boolean = "true".to_string();
    let result: Result<bool, _> = bool::from_str(&string_boolean);

    match result {
        Ok(value) => println!("String converted to boolean: {}", value),
        Err(_) => println!("Failed to convert string to boolean"),
    }
}
