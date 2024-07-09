fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Cerca il primo numero pari e raddoppia il valore
    let result = numbers.iter().find_map(|&x| {
        if x % 2 == 0 {
            Some(x * 2)
        } else {
            None
        }
    });

    match result {
        Some(value) => println!("Il primo numero pari raddoppiato è: {}", value),
        None => println!("Nessun numero pari trovato"),
    }
}
