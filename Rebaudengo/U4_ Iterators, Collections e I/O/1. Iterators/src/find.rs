fn main() {
    let numbers = vec![2, 4, 6, 7, 8, 9];

    // Trova il primo numero dispari nell'iteratore
    let first_odd = numbers.iter().find(|&x| *x % 2 != 0);

    match first_odd {
        Some(odd) => println!("Il primo numero dispari è: {}", odd),
        None => println!("Nessun numero dispari trovato."),
    }
}

