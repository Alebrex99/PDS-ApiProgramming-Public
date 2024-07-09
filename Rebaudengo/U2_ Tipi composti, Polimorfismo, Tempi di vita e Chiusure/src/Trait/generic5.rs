#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}
impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }
}
fn main() {
    // Crea una coppia di interi
    let integer_pair = Pair::new(10i32, 20i32);
    println!("Integer pair: {:?}", integer_pair);

    // Crea una coppia di stringhe
    let string_pair = Pair::new("Hello".to_string(), "World".to_string());
    println!("String pair: {:?}", string_pair);

    // Crea una coppia di valori misti
    let mixed_pair = Pair::new(10i32, "Hello".to_string());
    println!("Mixed pair: {:?}", mixed_pair);
}
