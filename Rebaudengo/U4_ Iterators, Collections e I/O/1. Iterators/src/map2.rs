fn main() {
    let words = vec!["hello", "world", "how", "are", "you"];

    // Creiamo un iteratore dal vettore
    // applichiamo il metodo map per ottenere la lunghezza di ogni parola

    let word_uppercase_iter = words.iter().map(|word| word.len());

    // Stampiamo i risultati
    for name in word_uppercase_iter {
        println!("{}", name);
    }
  
    // applichiamo il metodo map per ottenere la conversione di ogni parola
    let word_uppercase_iter = words.iter().map(|w| w.to_uppercase());

    // Stampiamo i risultati
    for name in word_uppercase_iter {
        println!("{}", name);
    }
}
