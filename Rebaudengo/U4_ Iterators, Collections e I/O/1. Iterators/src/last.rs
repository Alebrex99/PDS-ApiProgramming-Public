fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Ottieni l'ultimo numero pari
    let last_even = numbers
		.iter()
		.filter(|&x| x % 2 == 0)
		.last();

    match last_even {
        Some(&number) => println!("L'ultimo numero pari è: {}", number),
        None => println!("Non ci sono numeri pari"),
    }
}
