fn main() {
    let strings = vec!["programming", "computer"];

    // Stampa la lunghezza di ciascuna stringa, gestendo gli errori
    let result = strings.iter().try_for_each(|s| -> Result<(), ()> {
        if s.len() > 5 {
            println!("La lunghezza di '{}' è maggiore di 5.", s);
            Ok(())
        } else {
            Err(()) // Interrompe l'iterazione se la lunghezza è minore o uguale a 5
        }
    });
    // Controlla il risultato dell'iterazione
    match result {
        Ok(()) => println!("Tutte le stringhe hanno una lunghezza maggiore di 5."),
        Err(()) => println!("Almeno una stringa ha una lunghezza minore o uguale a 5."),
    }
}