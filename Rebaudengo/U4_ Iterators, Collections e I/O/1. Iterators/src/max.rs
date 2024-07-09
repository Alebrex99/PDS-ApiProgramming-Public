fn main() {
    let numbers = vec![10, 30, 20, 50, 40];

    // Trova il massimo elemento nell'iteratore
    let max_number = numbers.iter().max();

    match max_number {
        Some(max) => println!("Il massimo elemento è: {}", max),
        None => println!("L'iteratore è vuoto."),
    }
}