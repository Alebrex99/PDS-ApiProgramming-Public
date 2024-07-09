use std::collections::HashMap;

fn main() {
    // Creiamo una HashMap con alcune voci di esempio
    let mut scores = HashMap::new();
    scores.insert("Alice", 42);
    scores.insert("Bob", 69);
    scores.insert("Charlie", 87);

    // Iteriamo sui valori mutabili della HashMap e li modifichiamo
    for (_, score) in scores.iter_mut() {
        *score += 10;
    }

    // Stampiamo i nuovi punteggi
    for (name, score) in scores.iter() {
        println!("{}: {}", name, score);
    }
}
