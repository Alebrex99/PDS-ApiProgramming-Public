fn main() {
    let words = vec!["hello", "world", "rust"];

    // Otteniamo un iteratore che produce copie delle stringhe
    let cloned_iter = words.iter().cloned();

    // Stampiamo ogni parola nel nuovo iteratore
    for word in cloned_iter {
        println!("Parola: {}", word);
    }
    for word in words {
        println!("Parola: {}", word);
    }

}
