use std::collections::BTreeMap;

fn main() {
    // Creiamo una BTreeMap con alcuni punteggi di esempio
    let mut scores = BTreeMap::new();
    scores.insert("Alice", 42);
    scores.insert("Bob", 69);
    scores.insert("Charlie", 87);

    // Incrementiamo il punteggio di Mark  di 5 punti, se esiste
    match scores.entry("Mark") {
        // Se la voce esiste, aggiungiamo 10 al punteggio esistente
        std::collections::btree_map::Entry::Occupied(mut entry) => {
            *entry.get_mut() += 5;
            println!("Il nuovo punteggio di Mark è: {}", entry.get());
        }
        // Se la voce non esiste, inseriamo una nuova voce con il punteggio 10
        std::collections::btree_map::Entry::Vacant(entry) => {
            entry.insert(5);
            println!("Abbiamo inserito un nuovo punteggio per Mark.");
        }
    }

    // Incrementiamo il punteggio di Alice di 10 punti, se esiste
    match scores.entry("Alice") {
        // Se la voce esiste, aggiungiamo 10 al punteggio esistente
        std::collections::btree_map::Entry::Occupied(mut entry) => {
            *entry.get_mut() += 10;
            println!("Il nuovo punteggio di Alice è: {}", entry.get());
        }
        // Se la voce non esiste, inseriamo una nuova voce con il punteggio 10
        std::collections::btree_map::Entry::Vacant(entry) => {
            entry.insert(10);
            println!("Abbiamo inserito un nuovo punteggio per Alice.");
        }
    }

    // Stampa i punteggi aggiornati
    println!("Punteggi aggiornati:");
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
}
