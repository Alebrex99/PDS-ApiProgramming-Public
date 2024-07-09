fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    // Otteniamo il terzo elemento del vettore
    match numbers.iter().nth(2) {
        Some(&number) => println!("Terzo elemento: {}", number),
        None => println!("Nessun elemento trovato all'indice specificato."),
    }

    // Otteniamo il secondo elemento del vettore
    match numbers.iter().nth(1) {
        Some(&number) => println!("Secondo elemento: {}", number),
        None => println!("Nessun elemento trovato all'indice specificato."),
    }
}
