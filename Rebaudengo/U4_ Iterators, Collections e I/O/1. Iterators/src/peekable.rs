fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut peekable_numbers = numbers.iter().peekable();
    // Controlla se c'è un elemento successivo e lo stampa senza consumarlo
    if let Some(&next_number) = peekable_numbers.peek() {
        println!("Elemento successivo: {}", next_number);
    }
    // Ora si può consumare l'elemento
    if let Some(next_number) = peekable_numbers.next() {
        println!("Elemento consumato: {}", next_number);
    }
    for number in peekable_numbers {
        println!("Elemento successivo: {}", number); // Stampa gli elementi rimanenti
    }
}
