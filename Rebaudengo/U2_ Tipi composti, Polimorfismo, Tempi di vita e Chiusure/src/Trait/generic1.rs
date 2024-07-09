fn get_value<T>(condition: bool, value: T) -> Option<T> {
    if condition {
        Some(value)
    } else {
        None
    }
}

fn main() {
    let condition = true;
    let value = "Hello, world!";
    match get_value(condition, value) {
        Some(v) => println!("Value: {}", v),
        None => println!("No value found"),
    }
}