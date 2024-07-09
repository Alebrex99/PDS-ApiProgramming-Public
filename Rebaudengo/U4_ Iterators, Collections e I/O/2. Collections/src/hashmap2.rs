
use std::collections::HashMap;

fn main() {
    // Creazione di una HashMap
    let mut scores = HashMap::new();
    scores.insert("Alice", 100);
    scores.insert("Bob", 90);
    scores.insert("Charlie", 80);

    // Utilizzo del metodo keys per ottenere un iteratore sugli elementi delle chiavi
    let keys_iter = scores.keys();

    // Iterazione sugli elementi delle chiavi e stampa dei valori associati
    println!("Valori associati alle chiavi nella HashMap:");
    for key in keys_iter {
        if let Some(value) = scores.get(key) {
            println!("Chiave: {}, Valore: {}", key, value);
        }
    }
}