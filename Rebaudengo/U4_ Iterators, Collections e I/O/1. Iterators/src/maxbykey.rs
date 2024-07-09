fn main() {
    let words = vec!["hello", "world", "rust", "programming"];

    // Trova la stringa con il maggior numero di caratteri
    let longest_word = words.iter().max_by_key(|word| word.len());

    match longest_word {
        Some(longest) => println!("La stringa più lunga è: {}", longest),
        None => println!("L'iteratore è vuoto."),
    }
}
