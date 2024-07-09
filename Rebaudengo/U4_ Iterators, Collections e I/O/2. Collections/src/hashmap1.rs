use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // Inserimento di coppie chiave-valore
    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"), 85);
    scores.insert(String::from("Charlie"), 90);

    // Accesso ai valori tramite chiave
    println!("Punteggio di Alice:{}", scores["Alice"]);

    // Aggiornamento di un valore
    scores.insert(String::from("Bob"), 90);

    // Stampa di tutti i punteggi
    for (name, score) in &scores {
        println!("{} ha ottenuto {}", name, score);
    }

    // Verifica se una chiave esiste nella HashMap
    if !scores.contains_key("David") {
        println!("David non ha un punteggio registrato");
    }

// Rimozione di una coppia chiave-valore
    scores.remove("Bob");
// Verifica se la chiave è presente dopo la rimozione
    if scores.contains_key("Bob") {
        println!("Bob ha ancora un punteggio registrato.");
    } else {
        println!("Bob non ha più un punteggio registrato.");
    }

// Controllo della lunghezza della HashMap
    println!("Numero di punteggi: {}", scores.len());

// Iterazione sui valori della HashMap
    for score in scores.values() {
        println!("Punteggio: {}", score);
    }

// Iterazione sui riferimenti della HashMap
    for (name, score) in &scores {
        println!("{} ha ottenuto {}", name, score);
    }

// Rimozione di tutti gli elementi dalla HashMap
    scores.clear();

// Verifica se la HashMap è vuota
    if scores.is_empty() {
        println!("La HashMap è vuota.");
    }
}


