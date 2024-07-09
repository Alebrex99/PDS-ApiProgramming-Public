fn main() {
    let words = vec!["hello", "world", "rust", "programming"];

    // Verifichiamo se almeno una stringa ha una lunghezza maggiore di 5 caratteri
    let any_long_word = words.iter().any(|&word| word.len() > 5);

    if any_long_word {
        println!("Almeno una parola ha una lunghezza maggiore di 5 caratteri.");
    } else {
        println!("Nessuna parola ha una lunghezza maggiore di 5 caratteri.");
    }
}
