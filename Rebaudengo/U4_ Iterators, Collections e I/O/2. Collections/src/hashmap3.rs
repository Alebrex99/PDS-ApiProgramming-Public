use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // Inserimento di una coppia chiave-valore utilizzando il metodo entry
    scores.entry("Alice").or_insert(100);
    scores.entry("Bob").or_insert(90);
    scores.entry("Charlie").or_insert(80);

    // Aggiornamento del punteggio di Alice usando il metodo entry
    let alice_entry = scores.entry("Alice");
    match alice_entry {
        Entry::Occupied(mut entry) => {
            *entry.get_mut() += 10; // Aggiunge 10 al punteggio di Alice
        }
        Entry::Vacant(_) => {
            println!("Alice non trovata"); // Alice non è presente nella mappa
        }
    }

    // Stampa della HashMap aggiornata
    println!("HashMap: {:?}", scores);
}
